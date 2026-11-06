#!/usr/bin/env python3
"""Run the check chain and print the Checks paragraph for a commit message.

The commit's Checks paragraph is this script's output (decision 0058),
so a message cannot claim more or less than the checks showed. The
script runs, in order:

- tools/similarity.py --fail-on-flag on the default directories, when
  TIDAL_REFERENCE names the reference corpus or --reference is given,
  writing the report to --similarity-out if given,
- tools/check_specification.py,
- tools/question_index.py --check,
- tools/frontend_check.py, unless --no-frontend.

A new similarity report changes the similarity column of the status
page, so run tools/status_table.py after the first run that writes one,
and then run this script again.

It prints one paragraph that gives each check's own summary line, and
exits 1 if any check failed. Use it as `tools/checks.py > FILE && git
commit -F MSG` with the paragraph pasted from FILE, or append FILE to
the message.

Usage:
    tools/checks.py [--reference DIR] [--similarity-out FILE] [--no-frontend]
"""

import argparse
import os
import subprocess
import sys
import textwrap
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def run(argv: list[str], out: Path | None = None) -> tuple[int, str]:
    """Run one check; return its exit code and its last line that is not a warning."""
    proc = subprocess.run([sys.executable, "-I", *argv], cwd=ROOT, capture_output=True, text=True)
    text = proc.stdout + proc.stderr
    if out is not None:
        out.write_text(proc.stdout)
    lines = [l for l in text.splitlines() if l.strip() and not l.startswith("warning")]
    return proc.returncode, lines[-1] if lines else ""


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--reference", default=os.environ.get("TIDAL_REFERENCE"))
    ap.add_argument("--similarity-out", type=Path)
    ap.add_argument("--no-frontend", action="store_true")
    args = ap.parse_args()
    parts, failed = [], False
    if args.reference:
        out = args.similarity_out.resolve() if args.similarity_out else None
        code, line = run(["tools/similarity.py", "--fail-on-flag", "--reference", args.reference], out)
        failed |= code != 0
        where = f" ({out.relative_to(ROOT)})" if out else ""
        parts.append(f"similarity.py --fail-on-flag on the default directories reports \"{line}\"{where}")
    else:
        parts.append("similarity.py did not run (no reference corpus given)")
    code, line = run(["tools/check_specification.py"])
    failed |= code != 0
    parts.append(f"check_specification.py reports \"{line}\"")
    code, line = run(["tools/question_index.py", "--check"])
    failed |= code != 0
    parts.append(f"question_index.py --check reports \"{line}\"")
    if not args.no_frontend:
        proc = subprocess.run([sys.executable, "-I", "tools/frontend_check.py"], cwd=ROOT,
                              capture_output=True, text=True)
        failed |= proc.returncode != 0
        summary = next((l for l in proc.stdout.splitlines() if " pass, " in l), "no summary")
        parts.append(f"frontend_check.py reports \"{summary.removeprefix('frontend: ')}\"")
    text = "Checks: " + "; ".join(parts) + "." + (" At least one check failed." if failed else "")
    print(textwrap.fill(text, 72))
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
