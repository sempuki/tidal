"""X23 lexical scan. Lists files under tests/ that declare a type whose alignment is
written as more than 16 (alignas(N) with N > 16, or a multiple of
__STDCPP_DEFAULT_NEW_ALIGNMENT__), and reports for each file:
- whether the file declares a class-scope operator new or operator delete,
- whether the file has a new expression or a delete expression.
The scan is lexical; every hit is reviewed by hand."""
import re, sys, pathlib
root = pathlib.Path(sys.argv[1])
big = re.compile(r"alignas\s*\(\s*(?:(\d+)|[^)]*__STDCPP_DEFAULT_NEW_ALIGNMENT__[^)]*)\)")
classop = re.compile(r"(static\s+)?(void\*?\s*)operator\s+(new|delete)(\[\])?\s*\(")
newexpr = re.compile(r"(?<![\w:])(::\s*)?new\b(?!\s*\()|(?<![\w:])(::\s*)?new\s*\(")
delexpr = re.compile(r"(?<![\w])delete\b(?!\s*\()")
for p in sorted(list(root.rglob("*.cpp")) + list(root.rglob("*.h")) + list(root.rglob("*.hpp"))):
    t = p.read_text(errors="replace")
    t2 = re.sub(r"//.*", "", t)
    ok = False
    for m in big.finditer(t2):
        if m.group(1) is None or int(m.group(1)) > 16:
            ok = True
    if not ok:
        continue
    has_op = bool(re.search(r"static\s+void\*?\s*operator\s+(new|delete)", t2)) or bool(re.search(r"\n\s+void\*?\s*operator\s+(new|delete)", t2))
    has_new = bool(re.search(r"(?<![\w])new\s+[A-Za-z_(]|new\s*\(", re.sub(r"operator\s+new", "", t2)))
    has_del = bool(re.search(r"(?<![\w])delete(\s*\[\s*\])?\s+[A-Za-z_*(]", re.sub(r"operator\s+delete", "", t2)))
    print(f"{p.relative_to(root.parent)}\tclass-op={'yes' if has_op else 'no'}\tnew={'yes' if has_new else 'no'}\tdelete={'yes' if has_del else 'no'}")
