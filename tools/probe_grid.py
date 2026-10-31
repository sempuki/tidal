#!/usr/bin/env python3
"""Compile a grid of probe programs on the compiler matrix and tabulate the outcomes.

A grid file (JSON) gives a program template and two lists of fill-ins.
Each cell substitutes one row and one column into the template, and every
selected compiler checks the program (compile only, as `accept` tests
do). Probes encode what they observe in the program itself, for example
with `static_assert`, so front-end-only compilers take part.

Grid file:

    {
      "prelude": "struct B {}; struct D : B {};",
      "template": "{prelude}\\nvoid f({col});\\nvoid g() {{ {row} }}",
      "rows": {"int lvalue": "int x = 0; f(x);", "D prvalue": "f(D{});"},
      "cols": {"long": "long", "B": "B"},
      "flags": []
    }

The template uses str.format: {prelude}, {row}, and {col}; double any
other brace. A grid can give "pairs" in place of "cols": a set of
fill-ins whose every unordered pair becomes a column, with {a} and {b}
in the template. A ranking probe uses pairs, for example two overloads
f({a}) and f({b}) that return different types. The output is a Markdown table with one row per grid row. A
cell shows "accept" or "reject" when every compiler agrees, and lists
the compilers on each side when they split.

Usage:
    tools/probe_grid.py GRID.json [--compilers baseline|all|ID,ID]
                       [--json OUT.json] [--errors]
"""

import argparse
import concurrent.futures
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import matrix  # noqa: E402
from matrix import BUILD, MOUNT  # noqa: E402


def columns(grid: dict) -> dict[str, dict[str, str]]:
    """Column label -> fill-ins. "cols" gives {col}; "pairs" gives {a} and {b}
    for every unordered pair of its entries, so a grid covers every pair."""
    if "pairs" in grid:
        items = list(grid["pairs"].items())
        return {f"{la} / {lb}": {"a": a, "b": b}
                for i, (la, a) in enumerate(items) for lb, b in items[i + 1:]}
    return {label: {"col": col} for label, col in grid["cols"].items()}


def cells(grid: dict) -> list[tuple[str, str, str]]:
    """(row label, column label, program) for every cell."""
    out = []
    for r, row in grid["rows"].items():
        for c, fill in columns(grid).items():
            out.append((r, c, grid["template"].format(prelude=grid.get("prelude", ""), row=row, **fill)))
    return out


def slug(text: str) -> str:
    return re.sub(r"[^A-Za-z0-9]+", "-", text).strip("-").lower() or "x"


def probe(c: matrix.Compiler, name: str, index: int, r: str, col: str, program: str,
          flags: list[str]) -> tuple[str, str]:
    # The index keeps labels that differ only in punctuation (int& and int&&) apart.
    rel = Path("probes") / name / f"{index:04d}-{slug(r)}--{slug(col)}"
    host = BUILD / c.id / rel
    host.parent.mkdir(parents=True, exist_ok=True)
    src = host.with_suffix(".cpp")
    src.write_text(program + "\n")
    if c.image:
        csrc, cout = f"{MOUNT}/build/{rel}.cpp", f"{MOUNT}/build/{rel}"
    else:
        csrc, cout = str(src), str(host)
    if any(any(re.fullmatch(p.replace("*", ".*"), f) for p in c.unsupported_flags) for f in flags):
        return "skip", ""
    proc = c.run(c.argv(c.check, csrc, cout, flags))
    if proc.returncode == 0:
        return "accept", ""
    errors = [l for l in (proc.stderr + proc.stdout).splitlines() if "error" in l]
    return "reject", errors[0].strip() if errors else ""


def summary(results: dict[str, tuple[str, str]]) -> str:
    sides: dict[str, list[str]] = {}
    for cid, (outcome, _) in results.items():
        if outcome != "skip":
            sides.setdefault(outcome, []).append(cid)
    if len(sides) == 1:
        return next(iter(sides))
    return "; ".join(f"{o}: {', '.join(ids)}" for o, ids in sorted(sides.items()))


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("grid", type=Path)
    ap.add_argument("--compilers", default="baseline")
    ap.add_argument("--json", type=Path, help="write every outcome and first error line as JSON")
    ap.add_argument("--errors", action="store_true", help="print each rejecting compiler's first error")
    args = ap.parse_args()

    grid = json.loads(args.grid.read_text())
    name = slug(args.grid.stem)
    flags = grid.get("flags", [])
    compilers = matrix.select(args.compilers)
    todo = cells(grid)
    results: dict[tuple[str, str], dict[str, tuple[str, str]]] = {(r, c): {} for r, c, _ in todo}
    try:
        for c in compilers:
            c.start()
        with concurrent.futures.ThreadPoolExecutor(max_workers=matrix.SHELLS * max(1, len(compilers))) as pool:
            jobs = {pool.submit(probe, c, name, i, r, col, program, flags): (r, col, c.id)
                    for i, (r, col, program) in enumerate(todo) for c in compilers}
            for job in concurrent.futures.as_completed(jobs):
                r, col, cid = jobs[job]
                results[(r, col)][cid] = job.result()
    finally:
        for c in compilers:
            c.stop()

    cols = list(columns(grid))
    print("| | " + " | ".join(cols) + " |")
    print("|---|" + "---|" * len(cols))
    for r in grid["rows"]:
        print(f"| {r} | " + " | ".join(summary(results[(r, col)]) for col in cols) + " |")
    if args.errors:
        for (r, col), res in results.items():
            for cid, (outcome, err) in sorted(res.items()):
                if outcome == "reject":
                    print(f"{r} / {col} / {cid}: {err}")
    if args.json:
        args.json.write_text(json.dumps(
            [{"row": r, "col": col, "results": {cid: list(v) for cid, v in res.items()}}
             for (r, col), res in results.items()], indent=1) + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
