# Setting up a machine

This page lists what a machine needs to run every Tidal check. The
commands are for Fedora 44, the reference host. Other distributions need
the same packages under their own names.

## Host packages

    sudo dnf install -y git gh python3 podman \
        clang compiler-rt gcc-c++ libubsan libasan

- `clang` and `gcc-c++` are the host compilers `clang-host` and
  `gcc-host` in `tools/compilers.json`. The matrix records their exact
  versions in `tests/RESULTS.md`.
- `compiler-rt`, `libubsan`, and `libasan` are the sanitizer runtimes
  that the sanitizer tests link against (decision 0028).
- `podman` runs the container compilers. Docker also works: set
  `TIDAL_CONTAINER_ENGINE=docker`.

## Rust

The clean-room front end in `frontend/` builds with stable Rust and no
crates (decision 0029):

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

## Container images

    tools/containers.py build

This builds EDG 7.0 from its open source release, GCC 14 and 16, and
Clang 18 and 22. The first build takes several minutes.

## Site

    python3 -m venv build/venv
    build/venv/bin/pip install -r tools/requirements-site.txt
    tools/build_site.py --strict

## Reference corpus

The similarity check needs WG21 document N4950 outside the repository
(`reports/README.md`). Only `tools/similarity.py` reads it.

    git clone --depth 1 --branch n4950 https://github.com/cplusplus/draft ~/tidal-reference/draft

## Check the machine

    tools/check_all.sh

It runs the spec checks, the full test matrix, the similarity check when
`TIDAL_REFERENCE` names the corpus (for example
`TIDAL_REFERENCE=~/tidal-reference/draft/source`), and the site build.
The first full run fills `build/cache` and takes the longest.
