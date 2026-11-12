"""Build and run the X26 run-time claims, and list the warnings of the X26
warning probes, on the compilers that build. Run from the repository root
under flock on the run lock.

1. Runs: reports/clean-room/X26/p/o1.cpp, m1.cpp, u1.cpp, f1.cpp, f1.cpp
   with -DTWO, s1.cpp, and t1.cpp with each define. Each exits 0 when its
   claim holds.
2. Warnings: w1.cpp under -Wunused-value -fsyntax-only (the line of each
   warning; line n holds case L(n-8)), t1.cpp and f1.cpp under -Wunused-value
   with each define, and m1.cpp under -Wall -Wextra.
"""
import re
import sys
from pathlib import Path
sys.path.insert(0, "tools")
import matrix  # noqa: E402
from matrix import BUILD, MOUNT  # noqa: E402

IDS = "clang-host,gcc-host,gcc-14,clang-18,gcc-16,clang-22"
X26 = Path("reports/clean-room/X26/p")


def place(c, text, rel):
    host = BUILD / c.id / "probes" / "writer15" / rel
    host.parent.mkdir(parents=True, exist_ok=True)
    copy = host.with_suffix(".cpp")
    copy.write_text(text)
    if c.image:
        return f"{MOUNT}/build/probes/writer15/{rel}.cpp", f"{MOUNT}/build/probes/writer15/{rel}"
    return str(copy.resolve()), str(host)


def build_and_run(c, text, rel):
    src, out = place(c, text, rel)
    b = c.run(c.argv(c.build, src, out, []))
    if b.returncode:
        err = [l for l in (b.stderr + b.stdout).splitlines() if "error" in l]
        return "build fails", (err[0] if err else "")[:160]
    r = c.run([out])
    return f"exit {r.returncode}", r.stdout.strip()


def warnings(c, text, rel, flags):
    src, out = place(c, text, rel)
    b = c.run(c.argv(c.check, src, out, flags))
    lines = sorted({int(m.group(1)) for m in re.finditer(r"\.cpp:(\d+):\d+: warning", b.stderr + b.stdout)})
    kinds = sorted(set(re.findall(r"\[(-W[\w-]+)\]", b.stderr + b.stdout)))
    status = "error" if b.returncode else "ok"
    return status, f"lines {lines} {' '.join(kinds)}".strip()


def defined(name, d):
    return (f"#define {d}\n" if d else "") + (X26 / f"{name}.cpp").read_text()


def main():
    runs, warns = [], []
    for c in matrix.select(IDS):
        c.start()
        try:
            for name, ds in [("o1", [""]), ("m1", [""]), ("u1", [""]), ("f1", ["", "TWO"]), ("s1", [""]),
                             ("t1", ["", "USE", "DEFER_CLASS", "DEFER"])]:
                for d in ds:
                    label = f"{name}.cpp" + (f" -D{d}" if d else "")
                    runs.append((label, c.id, *build_and_run(c, defined(name, d), f"run-{name}-{d or 'plain'}")))
            # A #define line shifts every line by one; w1 runs as written.
            warns.append(("w1.cpp -Wunused-value", c.id,
                          *warnings(c, defined("w1", ""), "w-w1", ["-Wunused-value"])))
            for d in ["", "USE", "DEFER_CLASS", "DEFER"]:
                warns.append((f"t1.cpp -Wunused-value" + (f" -D{d}" if d else ""), c.id,
                              *warnings(c, defined("t1", d), f"w-t1-{d or 'plain'}", ["-Wunused-value"])))
            for d in ["", "TWO"]:
                warns.append((f"f1.cpp -Wunused-value" + (f" -D{d}" if d else ""), c.id,
                              *warnings(c, defined("f1", d), f"w-f1-{d or 'plain'}", ["-Wunused-value"])))
            warns.append(("m1.cpp -Wall -Wextra", c.id, *warnings(c, defined("m1", ""), "w-m1", ["-Wall", "-Wextra"])))
        finally:
            c.stop()
    print("## Runs\n\n| Program | Compiler | Result | Output |\n|---|---|---|---|")
    for r in sorted(runs):
        print("| " + " | ".join(r) + " |")
    print("\n## Warnings\n\nIn w1.cpp, line n holds case L(n-8). With a define, line numbers of t1 and f1 are one more than in the file.\n")
    print("| Program | Compiler | Status | Warnings |\n|---|---|---|---|")
    for r in sorted(warns):
        print("| " + " | ".join(r) + " |")


if __name__ == "__main__":
    main()
