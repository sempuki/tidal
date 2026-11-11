"""Lexical scan for X24 (P22, labeled break and continue).

Reads every .cpp, .h, and .hpp file under the directory given as the
first argument (default: tests). It removes comments, string literals,
and character literals, then reports:

1. each `break` or `continue` that an identifier follows (the P22 form),
2. each identifier label that stands directly before a loop or a
   switch, after any further labels and attribute lists,
3. each `goto`, with the label it names,
4. for files under programs/, the counts of 2 and 3 per file.

The scan is lexical. It does not parse, so a hit is a place to read by
hand and a miss is not proof.
"""
import os
import re
import sys

ROOT = sys.argv[1] if len(sys.argv) > 1 else "tests"

TOKEN = re.compile(
    r"""
    (?P<ws>\s+)
  | (?P<lc>//[^\n]*)
  | (?P<bc>/\*.*?\*/)
  | (?P<raw>(?:u8|u|U|L)?R"(?P<d>[^ ()\\\t\n]{0,16})\(.*?\)(?P=d)")
  | (?P<str>(?:u8|u|U|L)?"(?:\\.|[^"\\\n])*")
  | (?P<chr>(?:u8|u|U|L)?'(?:\\.|[^'\\\n])*')
  | (?P<id>[A-Za-z_][A-Za-z_0-9]*)
  | (?P<num>\.?[0-9](?:[eEpP][+-]|[A-Za-z_0-9.']|)*)
  | (?P<pp>\#[^\n]*(?:\\\n[^\n]*)*)
  | (?P<p>::|\[\[|\]\]|.)
    """,
    re.S | re.X,
)

KEYWORDS_NOT_LABELS = {"default", "public", "private", "protected", "case"}
LOOPS = {"for", "while", "do", "switch"}


def tokens(text):
    line = 1
    out = []
    for m in TOKEN.finditer(text):
        kind = m.lastgroup
        if kind in ("ws", "lc", "bc", "pp", "raw", "str", "chr"):
            if kind in ("str", "chr", "raw"):
                out.append(("lit", "<lit>", line))
        elif kind == "d":
            pass
        else:
            out.append((kind, m.group(), line))
        line += m.group().count("\n")
    return out


def skip_attrs(toks, i):
    """Skip attribute lists [[ ... ]] starting at i."""
    while i < len(toks) and toks[i][1] == "[[":
        depth = 1
        i += 1
        while i < len(toks) and depth:
            if toks[i][1] == "[[":
                depth += 1
            elif toks[i][1] == "]]":
                depth -= 1
            i += 1
    return i


def scan(path):
    with open(path, encoding="utf-8", errors="replace") as f:
        toks = tokens(f.read())
    named_jumps, loop_labels, gotos = [], [], []
    for i, (k, v, line) in enumerate(toks):
        nxt = toks[i + 1] if i + 1 < len(toks) else None
        prev = toks[i - 1] if i > 0 else None
        if k == "id" and v in ("break", "continue") and nxt and nxt[0] == "id":
            named_jumps.append((line, v, nxt[1]))
        if k == "id" and v == "goto" and nxt and nxt[0] == "id":
            gotos.append((line, nxt[1]))
        # identifier label: id ':' not '::', previous token ends a statement
        if (
            k == "id"
            and v not in KEYWORDS_NOT_LABELS
            and nxt
            and nxt[1] == ":"
            and prev
            and prev[1] in (";", "{", "}", ":", "]]", ")")
        ):
            if prev[1] == ")":
                continue  # a bit-field, a ctor initializer, or a conditional
            if prev[1] == ":":
                # a chain of labels: the token before ':' must be a label
                pass
            j = i + 2
            # further labels and attributes
            while True:
                j = skip_attrs(toks, j)
                if (
                    j + 1 < len(toks)
                    and toks[j][0] == "id"
                    and toks[j + 1][1] == ":"
                    and toks[j][1] not in ("case", "default")
                ):
                    j += 2
                    continue
                break
            if j < len(toks) and toks[j][1] in LOOPS:
                loop_labels.append((line, v, toks[j][1]))
    return named_jumps, loop_labels, gotos


def main():
    files = []
    for d, _, names in os.walk(ROOT):
        for n in names:
            if n.endswith((".cpp", ".h", ".hpp")):
                files.append(os.path.join(d, n))
    files.sort()
    total_jumps = total_labels = total_gotos = 0
    program_rows = []
    print(f"files scanned: {len(files)}")
    print("== break or continue followed by an identifier ==")
    for p in files:
        nj, ll, gt = scan(p)
        for line, kw, name in nj:
            print(f"{p}:{line}: {kw} {name}")
        total_jumps += len(nj)
        total_labels += len(ll)
        total_gotos += len(gt)
        if "/programs/" in p and (ll or gt):
            program_rows.append((p, ll, gt))
    print(f"total: {total_jumps}")
    print("== label before a loop or switch, and goto, all files ==")
    for p in files:
        nj, ll, gt = scan(p)
        for line, name, stmt in ll:
            print(f"{p}:{line}: label {name} on {stmt}")
    print(f"labels on a loop or switch: {total_labels}")
    print(f"goto statements: {total_gotos}")
    print("== programs with a label on a loop or switch, or a goto ==")
    for p, ll, gt in program_rows:
        print(
            f"{p}: labels on loops {len(ll)} "
            f"({', '.join(n + ' on ' + s for _, n, s in ll)}); "
            f"goto {len(gt)} ({', '.join(n for _, n in gt)})"
        )
    print(f"programs: {len(program_rows)}")


if __name__ == "__main__":
    main()
