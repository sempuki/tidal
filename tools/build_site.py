#!/usr/bin/env python3
"""Build the Tidal site as HTML.

The site has three sources, which stay apart in the repository:
site/index.md (the front page), spec/ (the specification, the reference
material), and guides/ (tutorials and how-to guides). The script links
them into one Sphinx source tree in build/site-src, with spec/index.md as
the page `specification`, and builds with the configuration in site/.
The site is written to build/site. Install the pinned packages first:

    python3 -m venv build/venv
    build/venv/bin/pip install -r tools/requirements-site.txt

Usage:
    tools/build_site.py [--strict]
"""

import argparse
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def assemble() -> Path:
    """Link the three sources into build/site-src, and return that directory."""
    src = ROOT / "build" / "site-src"
    if src.exists():
        shutil.rmtree(src)
    src.mkdir(parents=True)
    (src / "index.md").symlink_to(ROOT / "site" / "index.md")
    (src / "guides").symlink_to(ROOT / "guides")
    for entry in (ROOT / "spec").iterdir():
        name = "specification.md" if entry.name == "index.md" else entry.name
        (src / name).symlink_to(entry)
    return src


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--strict", action="store_true", help="treat warnings as errors")
    args = ap.parse_args()
    venv = ROOT / "build" / "venv" / "bin" / "sphinx-build"
    sphinx = str(venv) if venv.exists() else "sphinx-build"
    cmd = [sphinx, "-q", "-b", "html", "-c", str(ROOT / "site"),
           str(assemble()), str(ROOT / "build" / "site")]
    if args.strict:
        cmd.insert(1, "-W")
    result = subprocess.run(cmd).returncode
    if result == 0:
        print(ROOT / "build" / "site" / "index.html")
    return result


if __name__ == "__main__":
    sys.exit(main())
