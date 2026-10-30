#!/usr/bin/env python3
"""Compare Tidal spec text with a reference corpus, printing only Tidal text.

Run by the checker (see docs/clean-room.md). The reference corpus is a
directory of .tex, .md, .txt, or .html files kept outside this repository.
The report names Tidal rules and quotes Tidal text only. It never prints
reference text, so it can be handed to the writer.

For each unit of Tidal text (a rule, or any other headed part of a section)
the report gives:
- longest: the longest run of words shared with the reference,
- overlap: the fraction of the unit's 5-word sequences found in the reference.

A unit is flagged when longest >= --run (default 8) or overlap > --overlap
(default 0.25).

Usage:
    tools/similarity.py --reference DIR [--spec spec/] [--out FILE] [--fail-on-flag]
"""

import argparse
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
HEADING = re.compile(r"^### ([A-Z]+-\d+) \(")
RULE_DIRECTIVE = re.compile(r"^:::\{rule\}\s+([A-Z]+-\d+)\s*$")
WORD = re.compile(r"[a-z0-9_]+")
LATEX_CMD = re.compile(r"\\[a-zA-Z@]+\*?(?:\[[^\]]*\])?")
HTML_TAG = re.compile(r"<[^>]+>")
N_SMALL = 5


def words(text: str) -> list[str]:
    text = LATEX_CMD.sub(" ", text)
    text = HTML_TAG.sub(" ", text)
    return WORD.findall(text.lower())


def ngrams(ws: list[str], n: int) -> set[tuple[str, ...]]:
    return {tuple(ws[i:i + n]) for i in range(len(ws) - n + 1)}


def load_reference(root: Path, n: int) -> tuple[set, set]:
    big, small = set(), set()
    for f in root.rglob("*"):
        if f.suffix in {".tex", ".md", ".txt", ".html", ".htm"} and f.is_file():
            ws = words(f.read_text(errors="replace"))
            big |= ngrams(ws, n)
            small |= ngrams(ws, N_SMALL)
    return big, small


def units(spec: Path):
    """Yield (label, text) for each rule and each other headed part of a section.

    A unit ends at the next heading of any level, so a rule's unit holds only
    that rule, and Evidence or Divergences text is checked on its own.
    """
    for f in sorted(spec.rglob("*.md")):
        label = f"{f.relative_to(spec)}:intro"
        buf = []
        for line in f.read_text().splitlines():
            if (m := HEADING.match(line)) or (m := RULE_DIRECTIVE.match(line)):
                yield label, "\n".join(buf)
                label, buf = m.group(1), []
            elif line.startswith(":tests:") or line.startswith(":kind:") or line == ":::":
                continue
            elif line.startswith("#"):
                yield label, "\n".join(buf)
                label, buf = f"{f.relative_to(spec)}:{line.lstrip('#').strip()[:40]}", []
            elif not line.startswith("Tests:"):
                buf.append(line)
        yield label, "\n".join(buf)


def longest_shared(ws: list[str], big: set, n: int) -> tuple[int, int]:
    """Return (length, start) of the longest run of ws covered by shared n-grams."""
    best, best_start = 0, 0
    run_start = None
    for i in range(len(ws) - n + 2):
        hit = i <= len(ws) - n and tuple(ws[i:i + n]) in big
        if hit and run_start is None:
            run_start = i
        elif not hit and run_start is not None:
            length = (i - 1 - run_start) + n
            if length > best:
                best, best_start = length, run_start
            run_start = None
    return best, best_start


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--reference", type=Path, required=True)
    ap.add_argument("--spec", type=Path, default=ROOT / "spec")
    ap.add_argument("--run", type=int, default=8)
    ap.add_argument("--overlap", type=float, default=0.25)
    ap.add_argument("--out", type=Path)
    ap.add_argument("--fail-on-flag", action="store_true", help="exit 1 if any unit is flagged")
    args = ap.parse_args()

    if args.reference.resolve().is_relative_to(ROOT):
        print("reference corpus must live outside the Tidal repository", file=sys.stderr)
        return 2

    big, small = load_reference(args.reference, args.run)
    lines = [
        f"Similarity report: run threshold {args.run} words, overlap threshold {args.overlap}",
        "",
    ]
    flagged = 0
    for label, text in units(args.spec):
        ws = words(text)
        if not ws:
            continue
        longest, start = longest_shared(ws, big, args.run)
        grams = ngrams(ws, N_SMALL)
        overlap = len(grams & small) / len(grams) if grams else 0.0
        flag = longest >= args.run or overlap > args.overlap
        flagged += flag
        lines.append(f"{'FLAG' if flag else 'ok  '} {label:<32} longest={longest:<3} overlap={overlap:.2f}")
        if longest >= args.run:
            lines.append(f"     shared run (Tidal text): {' '.join(ws[start:start + longest])}")
    lines += ["", f"{flagged} flagged"]

    report = "\n".join(lines) + "\n"
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report)
    print(report, end="")
    return 1 if args.fail_on_flag and flagged else 0


if __name__ == "__main__":
    sys.exit(main())
