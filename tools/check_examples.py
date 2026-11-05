#!/usr/bin/env python3
"""Compile the C++ examples of the specification pages.

The tool reads every ```cpp block under specification/ and compiles it
alone, as one translation unit, with the `check` command of each
selected compiler in tools/compilers.json. The command carries the
matrix flags (`-std=c++23 -pedantic-errors` for GCC and Clang,
`--c++23 --strict` for EDG). The default is the baseline compilers.

A MyST comment line just before the opening fence marks a block that
does not compile alone as written. The site does not show the comment.

    % tidal-example: fragment   the block needs context from the page
    % tidal-example: reject     every compiler rejects the block
    % tidal-example: diverges   the compilers differ on the block

A block with no marker must compile on every selected compiler. The
tool also fails when a marker no longer fits the results: a `fragment`
block that every compiler accepts, a `reject` block that a compiler
accepts, or a `diverges` block on which every compiler agrees.

Results are cached in build/cache/examples/, keyed by the block text,
the compiler's identity, and its command template. A run whose results
are all cached starts no container.
documentation/specification-format.md describes the markers.

The tool also runs the clean-room front end (`tidal-fe check`, decision
0029) on each block with no marker and each `reject` block. The front
end's status for a block is one of these:

    pass            the front end gives the block's answer
    FALSE-REJECT    a block with no marker, and the front end rejects it
    MISSED-REJECT   a `reject` block, and the front end accepts it
    unsupported     the front end stops at a construct it does not know
    crash           the front end ended in another way

A status other than `pass` does not fail the tool. The front end's
statuses are kept in reports/clean-room/rust-frontend/examples.tsv,
beside the baseline of tools/frontend_check.py. The tool fails when a
block's status moves to a status other than `pass`, from `pass` or from
another status, as tools/frontend_check.py does for tests. A block is
known by its page, its text, and its place among the blocks of that page
with the same text, so a block that moves on its page keeps its row,
and a block whose text changes is a new block. --update-frontend
rewrites the baseline; the tool also writes it when it does not exist.

Usage:
    tools/check_examples.py [--compilers baseline|containers|all|ID,ID]
                            [--no-cache] [-j N]
                            [--frontend-only | --no-frontend]
                            [--update-frontend] [--frontend-baseline FILE]
"""

import argparse
import itertools
import concurrent.futures
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import matrix  # noqa: E402
from matrix import BUILD, MOUNT, ROOT, Compiler  # noqa: E402

SPEC = ROOT / "specification"
CACHE = BUILD / "cache" / "examples"
MARKER = re.compile(r"^%\s*tidal-example:\s*(\S+)\s*$")
MARKS = {"fragment", "reject", "diverges"}
DEFAULT_COMPILERS = "baseline"
FRONTEND = ROOT / "frontend"
FRONTEND_BASELINE = ROOT / "reports" / "clean-room" / "rust-frontend" / "examples.tsv"
# The marks of the blocks that the front end checks, and the answer each
# one expects: True for reject.
FRONTEND_MARKS = {None: False, "reject": True}
RULE = re.compile(r": error \[([A-Z]+-\d+)")
# Bump when compile_block() changes how it computes a result.
OUTCOME_VERSION = 1
_names = itertools.count()


@dataclass(eq=False)
class Block:
    page: Path
    line: int          # line of the opening fence, 1-based
    mark: str | None   # None or one of MARKS
    text: str

    @property
    def name(self) -> str:
        return f"{self.page.relative_to(ROOT)}:{self.line}"

    @property
    def digest(self) -> str:
        return hashlib.sha256(self.text.encode()).hexdigest()


def blocks(page: Path) -> tuple[list[Block], list[str]]:
    """Return the cpp blocks of a page, and the errors in its markers."""
    found, errors = [], []
    lines = page.read_text().splitlines()
    i = 0
    while i < len(lines):
        fence = re.match(r"^(`{3,})(.*)$", lines[i])
        if not fence:
            m = MARKER.match(lines[i])
            if m and not (i + 1 < len(lines) and lines[i + 1].startswith("```cpp")):
                errors.append(f"{page.relative_to(ROOT)}:{i + 1}: marker not just before a cpp block")
            i += 1
            continue
        ticks, info = fence.group(1), fence.group(2).strip()
        end = i + 1
        while end < len(lines) and lines[end].rstrip() != ticks:
            end += 1
        if info == "cpp":
            mark = None
            m = MARKER.match(lines[i - 1]) if i > 0 else None
            if m:
                mark = m.group(1)
                if mark not in MARKS:
                    errors.append(f"{page.relative_to(ROOT)}:{i}: unknown marker '{mark}'")
            found.append(Block(page, i + 1, mark, "\n".join(lines[i + 1:end]) + "\n"))
        i = end + 1
    return found, errors


def compile_block(block: Block, c: Compiler) -> tuple[bool, str]:
    """Compile one block; return (accepted, first error line)."""
    host_dir = BUILD / c.id / "examples"
    host_dir.mkdir(parents=True, exist_ok=True)
    name = f"{block.digest[:16]}-{os.getpid()}-{next(_names)}"
    host_src = host_dir / f"{name}.cpp"
    host_src.write_text(block.text)
    if c.image:
        src, out = f"{MOUNT}/build/examples/{name}.cpp", f"{MOUNT}/build/examples/{name}"
    else:
        src, out = str(host_src), str(host_dir / name)
    try:
        proc = c.run(c.argv(c.check, src, out, []), timeout=60)
    except subprocess.TimeoutExpired:
        return False, "timeout"
    finally:
        for f in (host_src, host_dir / f"{name}.o"):
            f.unlink(missing_ok=True)
    text = proc.stdout + proc.stderr
    first = next((ln for ln in text.splitlines() if "error" in ln.lower()), "")
    return proc.returncode == 0, first.strip()[:200]


def identity(c: Compiler) -> str:
    """A string that changes whenever the compiler does: the image ID of a
    container compiler, or the version output of a host compiler. Neither
    needs a running container."""
    if c.image:
        proc = subprocess.run(
            [matrix.engine(), "image", "inspect", "--format", "{{.Id}}", c.image_tag],
            capture_output=True, text=True)
        return proc.stdout.strip() or f"missing {c.image_tag}"
    proc = c.run(c.version)
    return (proc.stdout + proc.stderr).strip()


def cache_entry(block: Block, c: Compiler, ident: str) -> Path:
    key = hashlib.sha256(json.dumps(
        [block.text, c.id, ident, c.check, sorted(c.drop_flags), OUTCOME_VERSION]).encode()).hexdigest()
    return CACHE / f"{key}.json"


def cargo() -> str | None:
    found = shutil.which("cargo")
    home = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo")) / "bin" / "cargo"
    return found or (str(home) if home.exists() else None)


def block_keys(all_blocks: list[Block]) -> dict[Block, str]:
    """A key for each block: its page, a digest of its text, and its place
    among the blocks of that page with the same text."""
    seen: dict[tuple[str, str], int] = {}
    keys = {}
    for b in all_blocks:
        page = b.page.relative_to(ROOT).as_posix()
        n = seen.get((page, b.digest), 0)
        seen[(page, b.digest)] = n + 1
        keys[b] = f"{page}#{b.digest[:12]}" + (f"-{n + 1}" if n else "")
    return keys


def frontend_check(all_blocks: list[Block], jobs: int) -> dict[Block, tuple[str, str]] | None:
    """Run `tidal-fe check` on each block with no marker and each `reject`
    block. Return {block: (status, rule of the first error or "-")}, or
    None when cargo is missing."""
    cargo_bin = cargo()
    if not cargo_bin:
        return None
    build = subprocess.run([cargo_bin, "build", "--release", "-q"], cwd=FRONTEND,
                           capture_output=True, text=True)
    if build.returncode:
        sys.exit(f"frontend: build failed\n{build.stderr}")
    exe = FRONTEND / "target" / "release" / "tidal-fe"
    chosen = [b for b in all_blocks if b.mark in FRONTEND_MARKS]
    with tempfile.TemporaryDirectory(prefix="tidal-examples-") as tmp:
        def one(item):
            i, b = item
            src = Path(tmp) / f"block{i}.cpp"
            src.write_text(b.text)
            try:
                proc = subprocess.run([str(exe), "check", str(src)],
                                      capture_output=True, text=True, timeout=60)
            except subprocess.TimeoutExpired:
                return "crash", "-"
            m = RULE.search(proc.stderr)
            rule = m.group(1) if m and proc.returncode == 1 else "-"
            if proc.returncode not in (0, 1, 2):
                return "crash", "-"
            if proc.returncode == 2:
                return "unsupported", "-"
            rejected = proc.returncode == 1
            if rejected == FRONTEND_MARKS[b.mark]:
                return "pass", rule
            return ("FALSE-REJECT" if rejected else "MISSED-REJECT"), rule

        with concurrent.futures.ThreadPoolExecutor(jobs) as pool:
            out = list(pool.map(one, enumerate(chosen)))
    return dict(zip(chosen, out))


def read_frontend_baseline(path: Path) -> dict[str, str]:
    base = {}
    for line in path.read_text().splitlines():
        if line and not line.startswith("#"):
            status, key = line.split("\t")[:2]
            base[key] = status
    return base


def write_frontend_baseline(path: Path, rows: list[tuple[str, str, str, str]]) -> None:
    lines = ["# The clean-room front end's status for each page example with no marker",
             "# and each `reject` example: status, key (page#digest), line, rule of the",
             "# first error. tools/check_examples.py compares with this file and",
             "# rewrites it with --update-frontend."]
    lines += ["\t".join(r) for r in sorted(rows, key=lambda r: r[1])]
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines) + "\n")


def frontend_report(all_blocks: list[Block], args) -> int:
    """Run the front end on the blocks, compare with the baseline, and
    print the result. Return 1 when a block's status became worse."""
    results = frontend_check(all_blocks, args.j)
    if results is None:
        print("frontend: skipped (cargo not found, see documentation/setup.md)")
        return 0
    keys = block_keys(all_blocks)
    rows = [(s, keys[b], str(b.line), rule) for b, (s, rule) in results.items()]
    now = {keys[b]: s for b, (s, _) in results.items()}
    names = {keys[b]: b.name for b in results}
    path = Path(args.frontend_baseline)
    for b, (s, rule) in results.items():
        if s != "pass":
            print(f"frontend: {b.name}: {s}" + (f" (first error {rule})" if rule != "-" else ""))
    counts = {}
    for s in now.values():
        counts[s] = counts.get(s, 0) + 1
    summary = ", ".join(f"{n} {s}" for s, n in sorted(counts.items(), key=lambda x: -x[1]))
    if args.update_frontend or not path.exists():
        write_frontend_baseline(path, rows)
        print(f"frontend: {len(now)} examples ({summary}); baseline written to "
              f"{path.relative_to(ROOT) if path.is_relative_to(ROOT) else path}")
        return 0
    base = read_frontend_baseline(path)
    worse = sorted(k for k, s in now.items() if k in base and s != base[k] and s != "pass")
    better = sorted(k for k, s in now.items() if k in base and s == "pass" and base[k] != "pass")
    new = [k for k in now if k not in base]
    gone = [k for k in base if k not in now]
    for k in worse:
        print(f"FAIL frontend: {names[k]}: {now[k]}, {base[k]} in the baseline")
    for k in better:
        print(f"frontend: {names[k]}: {now[k]}, {base[k]} in the baseline")
    print(f"frontend: {len(now)} examples ({summary}); {len(worse)} worse, "
          f"{len(better)} better, {len(new)} new "
          f"({sum(now[k] != 'pass' for k in new)} not passing), {len(gone)} gone")
    return 1 if worse else 0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--compilers", default=DEFAULT_COMPILERS)
    ap.add_argument("--no-cache", action="store_true")
    ap.add_argument("-j", type=int, default=os.cpu_count())
    which = ap.add_mutually_exclusive_group()
    which.add_argument("--frontend-only", action="store_true",
                       help="run only the front end, not the compilers")
    which.add_argument("--no-frontend", action="store_true",
                       help="run only the compilers, not the front end")
    ap.add_argument("--update-frontend", action="store_true",
                    help="rewrite the front end's baseline")
    ap.add_argument("--frontend-baseline", default=str(FRONTEND_BASELINE),
                    help="the front end's baseline file")
    args = ap.parse_args()

    all_blocks, problems = [], []
    for page in sorted(SPEC.rglob("*.md")):
        found, errors = blocks(page)
        all_blocks += found
        problems += errors

    frontend_worse = 0 if args.no_frontend else frontend_report(all_blocks, args)
    if args.frontend_only:
        for p in problems:
            print("FAIL " + p)
        return 1 if problems or frontend_worse else 0

    compilers = matrix.select(args.compilers)
    idents = {c: identity(c) for c in compilers}
    results, misses = {}, []
    for b in all_blocks:
        for c in compilers:
            entry = cache_entry(b, c, idents[c])
            if not args.no_cache and entry.exists():
                entry.touch()  # marks the entry as used
                results[(b, c)] = tuple(json.loads(entry.read_text()))
            else:
                misses.append((b, c, entry))

    # Start only the containers that have blocks to compile.
    needed = [c for c in compilers if any(m[1] is c for m in misses)]
    try:
        for c in needed:
            c.start()

        def work(job):
            b, c, entry = job
            result = compile_block(b, c)
            if result != (False, "timeout"):  # transient under load; not cached
                CACHE.mkdir(parents=True, exist_ok=True)
                entry.write_text(json.dumps(result))
            return result

        with concurrent.futures.ThreadPoolExecutor(args.j) as pool:
            for job, result in zip(misses, pool.map(work, misses)):
                results[(job[0], job[1])] = result
    finally:
        for c in needed:
            c.stop()

    counts = {None: 0, "fragment": 0, "reject": 0, "diverges": 0}
    for b in all_blocks:
        counts[b.mark if b.mark in counts else None] += 1
        accepted = {c: results[(b, c)][0] for c in compilers}
        if b.mark is None:
            for c in compilers:
                if not accepted[c]:
                    problems.append(f"{b.name}: {c.id} rejects the block: {results[(b, c)][1]}")
        elif b.mark == "reject":
            for c in compilers:
                if accepted[c]:
                    problems.append(f"{b.name}: marked reject, but {c.id} accepts the block")
        elif b.mark == "fragment" and all(accepted.values()):
            problems.append(f"{b.name}: marked fragment, but every compiler accepts the block")
        elif b.mark == "diverges" and len(set(accepted.values())) == 1:
            verb = "accepts" if all(accepted.values()) else "rejects"
            problems.append(f"{b.name}: marked diverges, but every compiler {verb} the block")

    for p in problems:
        print("FAIL " + p)
    ids = ", ".join(c.id for c in compilers)
    print(f"examples: {len(all_blocks)} blocks ({counts[None]} standalone, "
          f"{counts['fragment']} fragment, {counts['reject']} reject, "
          f"{counts['diverges']} diverges) on {ids}: {len(problems)} problems")
    return 1 if problems or frontend_worse else 0


if __name__ == "__main__":
    sys.exit(main())
