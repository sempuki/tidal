"""Build and run the round 13 writer's run-time probes on the compilers that
build. Run from the repository root under flock build/run_tests.lock.

1. X25 run claims: reports/clean-room/X25/p/a1.cpp, m1.cpp, and a4.cpp
   with -DRUN under -fsanitize=address,undefined; a4's main is changed
   to return the value it reads, and a4 runs twice, once reading the new
   element through the member name (exit 7) and once reading the old
   member (exit 3);
   a9.cpp, whose exit status is 1 when the test of validity is true.
   construct.h is inlined into a copy under build/.
2. P4 claims: the five P4 program tests under tests/programs/P4/ and the
   two P4 reductions, each built once with -Wall -Wextra -pedantic-errors
   -Werror and once with -fsanitize=undefined,address
   -fno-sanitize-recover=all; each must build and exit 0.
"""
import sys
from pathlib import Path
sys.path.insert(0, "tools")
import matrix  # noqa: E402
from matrix import BUILD, MOUNT  # noqa: E402

IDS = "clang-host,gcc-host,gcc-14,clang-18,gcc-16,clang-22"
X25 = Path("reports/clean-room/X25/p")
HEADER = (X25 / "construct.h").read_text()
SAN = ["-fsanitize=address,undefined", "-fno-sanitize-recover=all", "-fno-omit-frame-pointer"]
WARN = ["-Wall", "-Wextra", "-pedantic-errors", "-Werror"]
P4 = sorted(Path("tests/programs/P4").glob("*.cpp")) + sorted(Path("reports/clean-room/P4/reductions").glob("*.cpp"))


def build_and_run(c, text: str, rel: str, flags, args=()):
    out_host = BUILD / c.id / "probes" / "writer13" / rel
    out_host.parent.mkdir(parents=True, exist_ok=True)
    copy = out_host.with_suffix(".cpp")
    copy.write_text(text)
    if c.image:
        src, out = f"{MOUNT}/build/probes/writer13/{rel}.cpp", f"{MOUNT}/build/probes/writer13/{rel}"
    else:
        src, out = str(copy.resolve()), str(out_host)
    b = c.run(c.argv(c.build, src, out, flags))
    if b.returncode:
        err = [l for l in (b.stderr + b.stdout).splitlines() if "error" in l or "warning" in l]
        return "build fails", (err[0] if err else "")[:160]
    r = c.run([out, *args])
    text = r.stderr + r.stdout
    tag = next((k for k in ("runtime error", "AddressSanitizer") if k in text), "")
    return f"exit {r.returncode}", tag


def main():
    rows = []
    for c in matrix.select(IDS):
        c.start()
        try:
            for name, args, label in [("a1.cpp", (), "a1 -DRUN"), ("m1.cpp", (), "m1 -DRUN"),
                                      ("a4.cpp", (), "a4 -DRUN, read items[1]"),
                                      ("a4.cpp", ("x",), "a4 -DRUN, read head")]:
                text = "#define RUN\n" + (X25 / name).read_text().replace('#include "construct.h"\n', HEADER)
                text = text.replace("return g(argc > 1) == 7 ? 0 : 1;", "return g(argc > 1);")
                rows.append((c.id, f"X25 {label}, sanitizers",
                             *build_and_run(c, text, "x25-" + label.split()[0] + str(len(args)), SAN, args)))
            text = (X25 / "a9.cpp").read_text().replace('#include "construct.h"\n', HEADER)
            rows.append((c.id, "X25 a9 (exit 1: test of validity true)", *build_and_run(c, text, "x25-a9", [])))
            for p in P4:
                for tag, flags in (("warnings", WARN), ("sanitizers", SAN)):
                    rows.append((c.id, f"P4 {p.name}, {tag}",
                                 *build_and_run(c, p.read_text(), f"p4-{p.stem}-{tag}", flags)))
        finally:
            c.stop()
    print("| Compiler | Program | Result | Note |")
    print("|---|---|---|---|")
    for r in rows:
        print("| " + " | ".join(r) + " |")


if __name__ == "__main__":
    main()
