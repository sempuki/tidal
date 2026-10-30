"""The compiler matrix: loading tools/compilers.json and running commands.

Host compilers run directly. Containerized compilers run inside one
long-lived container each, started with the tests directory mounted
read-only and a per-compiler build directory mounted read-write.
"""

import json
import os
import shutil
import subprocess
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TESTS = ROOT / "tests"
BUILD = ROOT / "build"
CONFIG = ROOT / "tools" / "compilers.json"
IMAGE_PREFIX = "tidal-cxx-"
MOUNT = "/tidal"


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
    unsupported_flags: list[str] = field(default_factory=list)
    container: str | None = None

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
                args.append(a.replace("{src}", src).replace("{out}", out))
        return args

    def run(self, argv: list[str], timeout: float | None = None) -> subprocess.CompletedProcess:
        if self.container:
            argv = [engine(), "exec", self.container, *argv]
        return subprocess.run(argv, capture_output=True, text=True, timeout=timeout)

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

    def stop(self) -> None:
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
