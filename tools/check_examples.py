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
are all cached starts no container. The tool does not run the front
end. documentation/specification-format.md describes the markers.

Usage:
    tools/check_examples.py [--compilers baseline|containers|all|ID,ID]
                            [--no-cache] [-j N]
"""

import argparse
import itertools
import concurrent.futures
import hashlib
import json
import os
import re
import subprocess
import sys
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


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--compilers", default=DEFAULT_COMPILERS)
    ap.add_argument("--no-cache", action="store_true")
    ap.add_argument("-j", type=int, default=os.cpu_count())
    args = ap.parse_args()

    all_blocks, problems = [], []
    for page in sorted(SPEC.rglob("*.md")):
        found, errors = blocks(page)
        all_blocks += found
        problems += errors

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
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
