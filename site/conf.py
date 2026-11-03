# Sphinx configuration for the Tidal specification site.
# Build with tools/build_site.py. Sources are the MyST Markdown files in specification/.

import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(Path(__file__).parent / "_ext"))

project = "Tidal"
author = "The Tidal contributors"
copyright = "the Tidal contributors"

commit = subprocess.run(["git", "-C", str(ROOT), "rev-parse", "--short", "HEAD"],
                        capture_output=True, text=True).stdout.strip()
release = f"draft {commit}" if commit else "draft"

extensions = ["myst_parser", "tidal"]
myst_enable_extensions = ["colon_fence", "deflist"]
root_doc = "index"
exclude_patterns = []
# Pygments retries C++ it can't lex (such as [[assume(...)]]) in relaxed mode.
suppress_warnings = ["myst.header", "misc.highlighting_failure"]

html_theme = "furo"
html_title = "Tidal: a C++-compatible specification"
html_static_path = ["_static"]
# Pages that moved keep their old URLs as redirects.
html_extra_path = ["_redirects"]
html_css_files = ["tidal.css"]
html_show_sourcelink = False
html_theme_options = {
    "source_repository": "https://github.com/sempuki/tidal",
    "source_branch": "main",
    "source_directory": "specification/",
}

tidal_repository = "https://github.com/sempuki/tidal"
tidal_root = str(ROOT)
