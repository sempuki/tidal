"""Build and run the round 14 writer's run-time probes on the compilers that
build. Run from the repository root under flock on the run lock.

1. X24 run claims: reports/clean-room/X24/p/d1.cpp as written and with
   -DCONT, and c1.cpp, k1.cpp, l1.cpp, s1.cpp, and f1.cpp, each of which
   exits 0 when the claim holds.
2. X24-1 for other exits: writer-probes/d2.cpp with CASE 1 to 6.
Each program prints the order in which it makes and ends its objects.
"""
import sys
from pathlib import Path
sys.path.insert(0, "tools")
import matrix  # noqa: E402
from matrix import BUILD, MOUNT  # noqa: E402

IDS = "clang-host,gcc-host,gcc-14,clang-18,gcc-16,clang-22"
X24 = Path("reports/clean-room/X24/p")
D2 = Path("reports/clean-room/roadmap-14/writer-probes/d2.cpp")


def build_and_run(c, text: str, rel: str):
    out_host = BUILD / c.id / "probes" / "writer14" / rel
    out_host.parent.mkdir(parents=True, exist_ok=True)
    copy = out_host.with_suffix(".cpp")
    copy.write_text(text)
    if c.image:
        src, out = f"{MOUNT}/build/probes/writer14/{rel}.cpp", f"{MOUNT}/build/probes/writer14/{rel}"
    else:
        src, out = str(copy.resolve()), str(out_host)
    b = c.run(c.argv(c.build, src, out, []))
    if b.returncode:
        err = [l for l in (b.stderr + b.stdout).splitlines() if "error" in l]
        return "build fails", (err[0] if err else "")[:160]
    r = c.run([out])
    return f"exit {r.returncode}", r.stdout.strip()


def programs():
    yield "d1 (goto models break outer)", "x24-d1", (X24 / "d1.cpp").read_text()
    yield "d1 -DCONT (goto models continue outer)", "x24-d1-cont", "#define CONT\n" + (X24 / "d1.cpp").read_text()
    for name in ("c1", "k1", "l1", "s1", "f1"):
        yield f"{name} (exit 0 when the claim holds)", f"x24-{name}", (X24 / f"{name}.cpp").read_text()
    labels = {1: "return out of nested loops", 2: "throw out of nested loops", 3: "goto out of if with init and condition",
              4: "goto out of while with condition", 5: "goto out of switch with init", 6: "break out of for with init and condition"}
    for k, label in labels.items():
        yield f"d2 CASE {k}: {label}", f"d2-{k}", f"#define CASE {k}\n" + D2.read_text()


def main():
    rows = []
    for c in matrix.select(IDS):
        c.start()
        try:
            for label, rel, text in programs():
                rows.append((label, c.id, *build_and_run(c, text, rel)))
        finally:
            c.stop()
    print("| Program | Compiler | Result | Output |")
    print("|---|---|---|---|")
    for r in sorted(rows, key=lambda r: (r[0], r[1])):
        print("| " + " | ".join(r) + " |")


if __name__ == "__main__":
    main()
