"""The compiler matrix: loading tools/compilers.json and running commands.

Host compilers run directly. Containerized compilers run inside one
long-lived container each, started with the tests directory mounted
read-only and a per-compiler build directory mounted read-write.

A `podman exec` costs about a second or more, and podman runs the execs
for one container one at a time. So each container keeps a pool of shells
open, and a command is a line written to one of them. The command's output
goes to files in the build mount, and a marker line returns its status.
"""

import itertools
import json
import os
import queue
import shlex
import shutil
import subprocess
import time
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TESTS = ROOT / "tests"
BUILD = ROOT / "build"
CONFIG = ROOT / "tools" / "compilers.json"
IMAGE_PREFIX = "tidal-cxx-"
MOUNT = "/tidal"
# Shells per container. Each runs one command at a time.
SHELLS = int(os.environ.get("TIDAL_CONTAINER_SHELLS", min(8, os.cpu_count() or 1)))
_job_ids = itertools.count()


def engine() -> str:
    """The container engine: $TIDAL_CONTAINER_ENGINE, else docker, else podman."""
    if e := os.environ.get("TIDAL_CONTAINER_ENGINE"):
        return e
    for e in ("docker", "podman"):
        if shutil.which(e):
            return e
    raise SystemExit("no container engine found (install docker or podman)")


@dataclass(eq=False)
class Compiler:
    id: str
    family: str
    role: str
    check: list[str]
    version: list[str]
    build: list[str] | None = None
    image: dict | None = None
    unsupported_flags: list[str] = field(default_factory=list)  # glob patterns; a test using one is skipped
    container: str | None = None
    shells: queue.Queue | None = None

    @property
    def image_tag(self) -> str:
        return IMAGE_PREFIX + self.id

    @property
    def frontend_only(self) -> bool:
        return self.build is None

    def paths(self, test: Path) -> tuple[str, str, Path]:
        """Return (src, out) as seen by the compiler, and out on the host."""
        rel = test.relative_to(TESTS)
        host_out = BUILD / self.id / rel.with_suffix("")
        host_out.parent.mkdir(parents=True, exist_ok=True)
        if self.image:
            return f"{MOUNT}/tests/{rel}", f"{MOUNT}/build/{rel.with_suffix('')}", host_out
        return str(test), str(host_out), host_out

    def argv(self, template: list[str], src: str | list[str], out: str,
             flags: list[str]) -> list[str]:
        """Expand a command template. {src} may stand for several sources."""
        srcs = [src] if isinstance(src, str) else src
        args = []
        for a in template:
            if a == "{flags}":
                args += flags
            elif a == "{src}":
                args += srcs
            else:
                args.append(a.replace("{src}", srcs[0]).replace("{out}", out))
        return args

    def run(self, argv: list[str], timeout: float | None = None) -> subprocess.CompletedProcess:
        if self.shells:
            return self._run_in_shell(argv, timeout)
        if self.container:
            argv = [engine(), "exec", self.container, *argv]
        return subprocess.run(argv, capture_output=True, text=True, timeout=timeout)

    def _run_in_shell(self, argv: list[str], timeout: float | None) -> subprocess.CompletedProcess:
        job = f"{os.getpid()}-{next(_job_ids)}"
        io = BUILD / self.id / ".io"
        cmd = shlex.join(argv)
        if timeout:
            cmd = f"timeout -k 5 {int(timeout)} {cmd}"
        line = (f"{cmd} >{MOUNT}/build/.io/{job}.out 2>{MOUNT}/build/.io/{job}.err </dev/null; "
                f"echo \"done {job} $?\"\n")
        shell = self.shells.get()
        try:
            start = time.monotonic()
            shell.stdin.write(line)
            shell.stdin.flush()
            while True:
                reply = shell.stdout.readline()
                if not reply:
                    raise RuntimeError(f"{self.id}: container shell exited")
                if reply.startswith(f"done {job} "):
                    code = int(reply.split()[2])
                    break
        finally:
            self.shells.put(shell)
        out, err = io / f"{job}.out", io / f"{job}.err"
        stdout = out.read_text(errors="replace")
        stderr = err.read_text(errors="replace")
        out.unlink()
        err.unlink()
        if timeout and code in (124, 137) and time.monotonic() - start >= timeout:
            raise subprocess.TimeoutExpired(argv, timeout, stdout, stderr)
        return subprocess.CompletedProcess(argv, code, stdout, stderr)

    def identity(self) -> str:
        """A string that changes whenever the compiler does, for result caching."""
        proc = self.run(self.version)
        text = proc.stdout + proc.stderr
        if self.image:
            proc = subprocess.run(
                [engine(), "image", "inspect", "--format", "{{.Id}}", self.image_tag],
                capture_output=True, text=True,
            )
            text += proc.stdout
        return text.strip()

    def version_line(self) -> str:
        proc = self.run(self.version)
        return (proc.stdout or proc.stderr).splitlines()[0]

    def start(self) -> None:
        if not self.image:
            return
        e = engine()
        if subprocess.run([e, "image", "inspect", self.image_tag], capture_output=True).returncode != 0:
            raise SystemExit(f"image {self.image_tag} missing; run tools/containers.py build {self.id}")
        build = BUILD / self.id
        build.mkdir(parents=True, exist_ok=True)
        self.container = f"tidal-{self.id}-{os.getpid()}"
        subprocess.run(
            [e, "run", "-d", "--rm", "--name", self.container,
             "-v", f"{TESTS}:{MOUNT}/tests:ro,z", "-v", f"{build}:{MOUNT}/build:z",
             "-w", MOUNT, self.image_tag, "sleep", "infinity"],
            check=True, capture_output=True,
        )
        (build / ".io").mkdir(exist_ok=True)
        self.shells = queue.Queue()
        for _ in range(SHELLS):
            self.shells.put(subprocess.Popen(
                [e, "exec", "-i", "-w", MOUNT, self.container, "sh"],
                stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
                text=True, errors="replace"))

    def stop(self) -> None:
        if self.shells:
            while not self.shells.empty():
                shell = self.shells.get()
                shell.stdin.close()
                shell.wait()
            self.shells = None
        if self.container:
            subprocess.run([engine(), "rm", "-f", self.container], capture_output=True)
            self.container = None


def load() -> list[Compiler]:
    config = json.loads(CONFIG.read_text())
    return [Compiler(**{k: v for k, v in c.items()}) for c in config["compilers"]]


def select(spec: str) -> list[Compiler]:
    """Select compilers by role ("baseline", "report"), "containers", "all",
    or a comma-separated id list."""
    compilers = load()
    if spec == "all":
        return compilers
    if spec in {"baseline", "report"}:
        return [c for c in compilers if c.role == spec]
    if spec == "containers":
        return [c for c in compilers if c.image]
    wanted = spec.split(",")
    by_id = {c.id: c for c in compilers}
    missing = [w for w in wanted if w not in by_id]
    if missing:
        raise SystemExit(f"unknown compiler(s): {', '.join(missing)}")
    return [by_id[w] for w in wanted]
