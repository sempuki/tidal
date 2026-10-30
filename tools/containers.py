#!/usr/bin/env python3
"""Build and list container images for the compiler matrix.

Usage:
    tools/containers.py list
    tools/containers.py build [ID ...]     # default: every containerized compiler
"""

import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import matrix  # noqa: E402


def main() -> int:
    if len(sys.argv) < 2 or sys.argv[1] not in {"list", "build"}:
        print(__doc__)
        return 2
    compilers = [c for c in matrix.load() if c.image]
    e = matrix.engine()

    if sys.argv[1] == "list":
        for c in compilers:
            proc = subprocess.run(
                [e, "image", "inspect", "--format", "{{.Id}}", c.image_tag],
                capture_output=True, text=True,
            )
            state = proc.stdout.strip()[:19] if proc.returncode == 0 else "(not built)"
            print(f"{c.id:<12} {c.image_tag:<24} {state}")
        return 0

    wanted = set(sys.argv[2:])
    for c in compilers:
        if wanted and c.id not in wanted:
            continue
        args = []
        for k, v in c.image.get("args", {}).items():
            args += ["--build-arg", f"{k}={v}"]
        dockerfile = matrix.ROOT / "docker" / c.image["dockerfile"]
        print(f"building {c.image_tag} from {dockerfile.name}", flush=True)
        subprocess.run(
            [e, "build", "-q", "-f", str(dockerfile), *args, "-t", c.image_tag,
             str(dockerfile.parent)],
            check=True,
        )
    return 0


if __name__ == "__main__":
    sys.exit(main())
