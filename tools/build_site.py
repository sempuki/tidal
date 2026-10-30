#!/usr/bin/env python3
"""Build the Tidal specification as an HTML site.

Uses Sphinx with the configuration in site/ and the sources in spec/. The
site is written to build/site. Install the pinned packages first:

    python3 -m venv build/venv
    build/venv/bin/pip install -r tools/requirements-site.txt

Usage:
    tools/build_site.py [--strict]
"""

import argparse
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--strict", action="store_true", help="treat warnings as errors")
    args = ap.parse_args()
    venv = ROOT / "build" / "venv" / "bin" / "sphinx-build"
    sphinx = str(venv) if venv.exists() else "sphinx-build"
    cmd = [sphinx, "-q", "-b", "html", "-c", str(ROOT / "site"),
           str(ROOT / "spec"), str(ROOT / "build" / "site")]
    if args.strict:
        cmd.insert(1, "-W")
    result = subprocess.run(cmd).returncode
    if result == 0:
        print(ROOT / "build" / "site" / "README.html")
    return result


if __name__ == "__main__":
    sys.exit(main())
