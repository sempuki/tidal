"""Build and run the round 12 writer's run-time probes on the compilers that
build. Run from the repository root under flock build/run_tests.lock.

1. P3-5: reports/clean-room/P3/probes/dangling-stage.cpp under
   -fsanitize=address; the probe passes when the run fails with
   stack-use-after-scope.
2. The P3 program tests built with -Wall -Wextra -Werror; each must build
   and exit 0.
3. The I7 programs and reductions (reports/clean-room/I7/), built with no
   extra flags; the I7 report claims exit 0 on the host compilers, and a
   reject of p03 by Clang (Q-0239).
"""
import shutil, sys
from pathlib import Path
sys.path.insert(0, "tools")
import matrix  # noqa: E402
from matrix import BUILD, MOUNT  # noqa: E402

IDS = "clang-host,gcc-host,gcc-14,clang-18,gcc-16,clang-22"
DANGLING = Path("reports/clean-room/P3/probes/dangling-stage.cpp")
PROGRAMS = sorted(Path("tests/programs/P3").glob("*.cpp"))
I7 = sorted(Path("reports/clean-room/I7/programs").glob("*.cpp")) + sorted(Path("reports/clean-room/I7/reductions").glob("*.cpp"))


def build_and_run(c, src_host: Path, rel: str, flags):
    out_host = BUILD / c.id / "probes" / "writer12" / rel
    out_host.parent.mkdir(parents=True, exist_ok=True)
    if c.image:
        if src_host.is_relative_to(Path("tests")):
            src = f"{MOUNT}/{src_host}"
        else:
            copy = out_host.with_suffix(".cpp")
            shutil.copy(src_host, copy)
            src = f"{MOUNT}/build/probes/writer12/{rel}.cpp"
        out = f"{MOUNT}/build/probes/writer12/{rel}"
    else:
        src, out = str(src_host.resolve()), str(out_host)
    b = c.run(c.argv(c.build, src, out, flags))
    if b.returncode:
        err = [l for l in (b.stderr + b.stdout).splitlines() if "error" in l or "warning" in l]
        return "build fails", err[0] if err else ""
    r = c.run([out])
    text = r.stderr + r.stdout
    tag = "stack-use-after-scope" if "stack-use-after-scope" in text else ""
    return f"exit {r.returncode}", tag


def main():
    rows = []
    for c in matrix.select(IDS):
        c.start()
        try:
            rows.append((c.id, "P3-5 dangling-stage, -fsanitize=address",
                         *build_and_run(c, DANGLING, "dangling-stage",
                                        ["-fsanitize=address", "-fno-omit-frame-pointer"])))
            for p in PROGRAMS:
                rows.append((c.id, f"{p.name}, -Wall -Wextra -Werror",
                             *build_and_run(c, p, p.stem, ["-Wall", "-Wextra", "-Werror"])))
            for p in I7:
                rows.append((c.id, f"I7 {p.name}", *build_and_run(c, p, "i7-" + p.stem, [])))
        finally:
            c.stop()
    print("| Compiler | Program | Result | Note |")
    print("|---|---|---|---|")
    for r in rows:
        print("| " + " | ".join(r) + " |")


if __name__ == "__main__":
    main()
