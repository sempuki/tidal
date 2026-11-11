#!/usr/bin/env python3
"""X26 scan: classify the left operand of each comma operator in tests.

For each .cpp file given (or found under the given directories), the
scan runs clang++ -std=c++23 -fsyntax-only -Xclang -ast-dump with the
file's tidal-flags, and reads the text dump. For each built-in comma
(BinaryOperator ',') and each overloaded comma (CXXOperatorCallExpr
',') at a location in the file or a header beside it, the scan
classifies the left operand under the X26 rule:

  ok        void type, a call, an assignment, an increment or decrement,
            or a conditional or comma whose result parts are ok
  bad       anything else
  deferred  an operator expression whose operands are dependent

A comma inside decltype, sizeof, alignof, noexcept, typeid, or a
requires expression is marked "unevaluated" and is exempt. The scan also
lists each declaration of an operator, function and each CXXFoldExpr.
Output: one line per finding, "path:line:col verdict context kind".
"""
import os
import re
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

LOC = re.compile(r"(?:(/[^\s:<>,']+):(\d+):(\d+)|line:(\d+):(\d+)|col:(\d+))")
STRIP = {"ParenExpr", "ImplicitCastExpr", "ExprWithCleanups",
         "MaterializeTemporaryExpr", "CXXBindTemporaryExpr", "ConstantExpr"}
CALLS = {"CallExpr", "CXXMemberCallExpr", "CXXOperatorCallExpr"}
ASSIGN_OPS = {"'='"}
# X26_STRICT=1 models the alternative "void or a call only" (report 1.9).
STRICT = os.environ.get("X26_STRICT") == "1"
UNEVAL = {"DecltypeType", "UnaryExprOrTypeTraitExpr", "CXXNoexceptExpr",
          "RequiresExpr", "CXXTypeidExpr"}


class Node:
    __slots__ = ("kind", "line", "children", "parent", "file", "ln", "col")

    def __init__(self, kind, line, parent, file, ln, col):
        self.kind, self.line, self.parent = kind, line, parent
        self.children = []
        self.file, self.ln, self.col = file, ln, col


def parse(dump):
    root = Node("ROOT", "", None, None, 0, 0)
    stack = [(-1, root)]
    cur = [None, 0, 0]
    for raw in dump.splitlines():
        m = re.match(r"^([ |`\-]*)(\S.*)$", raw)
        if not m:
            continue
        depth = len(m.group(1))
        body = m.group(2)
        kind = body.split(" ", 1)[0]
        start = None
        rng = re.search(r"<([^<>]*(?:<[^<>]*>[^<>]*)*)>", body)
        for i, lm in enumerate(LOC.finditer(body)):
            if lm.group(1):
                cur = [lm.group(1), int(lm.group(2)), int(lm.group(3))]
            elif lm.group(4):
                cur[1], cur[2] = int(lm.group(4)), int(lm.group(5))
            else:
                cur[2] = int(lm.group(6))
            if start is None and rng and lm.start() >= rng.start():
                start = list(cur)
        if start is None:
            start = list(cur)
        while stack[-1][0] >= depth:
            stack.pop()
        parent = stack[-1][1]
        node = Node(kind, body, parent, start[0], start[1], start[2])
        parent.children.append(node)
        stack.append((depth, node))
    return root


def node_type(n):
    m = re.search(r"> '([^']*)'", n.line)
    return m.group(1) if m else ""


def op_of(n):
    m = re.search(r"'([^']*)'\s*$", n.line)
    return m.group(1) if m else ""


def strip(n):
    while n.kind in STRIP and n.children:
        n = n.children[0]
    return n


def classify(n):
    t = node_type(n)
    if re.fullmatch(r"(const )?(volatile )?void", t):
        return "ok"
    s = strip(n)
    t = node_type(s)
    if re.fullmatch(r"(const )?(volatile )?void", t):
        return "ok"
    k = s.kind
    if k in CALLS:
        if k == "CXXOperatorCallExpr" and s.children and \
                "UnresolvedLookupExpr" in strip(s.children[0]).kind:
            return "deferred"
        return "ok"
    if k == "CompoundAssignOperator":
        return "bad:strict" if STRICT else "ok"
    if k == "BinaryOperator":
        op = op_of(s)
        if op == "=":
            return "bad:strict" if STRICT else "ok"
        if op == "," and len(s.children) == 2:
            return classify(s.children[1])
        if t == "<dependent type>":
            return "deferred"
        return "bad:" + k + op
    if k == "UnaryOperator":
        if re.search(r"(prefix|postfix) '(\+\+|--)'", s.line):
            return "bad:strict" if STRICT else "ok"
        if t == "<dependent type>":
            return "deferred"
        return "bad:" + k + op_of(s)
    if k == "ConditionalOperator" and len(s.children) == 3:
        a, b = classify(s.children[1]), classify(s.children[2])
        if a == "ok" and b == "ok":
            return "ok"
        if "deferred" in (a, b) and not (a.startswith("bad") or b.startswith("bad")):
            return "deferred"
        return "bad:Conditional"
    if k == "CXXThrowExpr" or k == "CXXDeleteExpr":
        return "ok"
    return "bad:" + k


def walk(n, uneval, out, base):
    u = uneval or n.kind in UNEVAL
    if n.kind == "BinaryOperator" and op_of(n) == "," and len(n.children) == 2:
        out.append((n, "unevaluated" if u else classify(n.children[0]), "builtin"))
    elif n.kind == "CXXOperatorCallExpr" and op_of(n) == ",":
        out.append((n, "overloaded", "overloaded"))
    elif n.kind == "CXXFoldExpr":
        out.append((n, "fold", "fold"))
    elif n.kind in ("FunctionDecl", "CXXMethodDecl") and \
            re.search(r" operator, '", n.line):
        out.append((n, "decl", "operator,"))
    for c in n.children:
        walk(c, u, out, base)


def flags_of(path):
    flags = []
    try:
        for line in path.read_text(errors="replace").splitlines()[:12]:
            m = re.match(r"//\s*tidal-flags:\s*(.*)", line)
            if m:
                flags += m.group(1).split()
    except OSError:
        pass
    return [f for f in flags if not f.startswith(("-W", "-fsanitize", "-O"))]


def scan(path):
    path = path.resolve()
    cmd = ["clang++", "-std=c++23", "-fsyntax-only", "-ferror-limit=0",
           "-w", "-I", str(path.parent)] + flags_of(path) + \
          ["-Xclang", "-ast-dump", str(path)]
    try:
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=120)
    except subprocess.TimeoutExpired:
        return [f"{path}: timeout"]
    root = parse(r.stdout)
    out = []
    walk(root, False, out, path)
    lines, seen = [], {}
    tests = str(path.parent.resolve())
    for n, verdict, what in out:
        f = n.file or str(path)
        if not str(Path(f).resolve()).startswith(tests):
            continue
        key = (f, n.ln, n.col, what)
        prev = seen.get(key)
        rank = {"bad": 3, "deferred": 1, "ok": 2}
        if prev is not None:
            pv = prev.split(":")[0]
            if rank.get(verdict.split(":")[0], 0) <= rank.get(pv, 0):
                continue
        seen[key] = verdict
    for (f, ln, col, what), verdict in sorted(seen.items()):
        lines.append(f"{f}:{ln}:{col} {what} {verdict}")
    return lines


def main():
    files = []
    for a in sys.argv[1:]:
        p = Path(a)
        if p.is_dir():
            files += sorted(p.rglob("*.cpp"))
        else:
            files.append(p)
    with ThreadPoolExecutor(24) as ex:
        for res in ex.map(scan, files):
            for line in res:
                print(line)


if __name__ == "__main__":
    main()
