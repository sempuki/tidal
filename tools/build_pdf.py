#!/usr/bin/env python3
"""Build the Tidal specification as a PDF.

Sections are taken in the order they are linked from spec/README.md.
They are concatenated, converted to Typst with pandoc, and compiled with
typst. The title page records the git commit the PDF was built from.

Usage:
    tools/build_pdf.py [--out build/tidal.pdf]
"""

import argparse
import datetime
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SPEC = ROOT / "spec"
STYLE = ROOT / "docs" / "pdf" / "style.typ"
LINK = re.compile(r"\]\(([^)#]+\.md)\)")


def sections() -> list[Path]:
    return [SPEC / m for m in LINK.findall((SPEC / "README.md").read_text())]


def section_text(path: Path) -> str:
    text = path.read_text()
    # Links between sections point at .md files; in one PDF they become plain text.
    text = re.sub(r"\[([^\]]+)\]\([^)]*\.md(#[^)]*)?\)", r"\1", text)
    text = re.sub(r"\[([^\]]+)\]\(#[^)]*\)", r"\1", text)
    # Show test paths as code, one list per rule.
    text = re.sub(r"^Tests: (.*)$", lambda m: "*Tests:* " + ", ".join(
        f"`{s.strip()}`" for s in m.group(1).split(",")), text, flags=re.M)
    return text


def git(*args: str) -> str:
    return subprocess.run(["git", "-C", str(ROOT), *args], capture_output=True,
                          text=True).stdout.strip()


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", type=Path, default=ROOT / "build" / "tidal.pdf")
    args = ap.parse_args()

    commit = git("rev-parse", "--short", "HEAD")
    if git("status", "--porcelain", "spec"):
        commit += " (with uncommitted spec changes)"
    body = "\n\n".join(section_text(p) for p in sections())

    work = args.out.parent
    work.mkdir(parents=True, exist_ok=True)
    md = work / "tidal.md"
    typ = work / "tidal.typ"
    md.write_text(body)
    subprocess.run(
        ["pandoc", str(md), "-f", "gfm", "-t", "typst", "--standalone",
         "-M", "title=Tidal: Expressions and Statements",
         "-M", f"subtitle=Draft built from commit {commit}",
         "-M", f"date={datetime.date.today().isoformat()}",
         "--toc", "--toc-depth=2",
         "-V", "papersize=us-letter", "-V", "section-numbering=1.1",
         "--include-in-header", str(STYLE),
         "-o", str(typ)],
        check=True,
    )
    subprocess.run(["typst", "compile", str(typ), str(args.out)], check=True)
    print(args.out)
    return 0


if __name__ == "__main__":
    sys.exit(main())
