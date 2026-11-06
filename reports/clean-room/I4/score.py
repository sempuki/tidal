"""Score conditional.py (frozen in Phase 1) against the Tidal tests.

Each case is one assertion of a test under tests/expressions/conditional/
(the other tests that cite a COND rule hold only int operands already
covered here). Expected values come from the test's tidal-expect line,
not from a compiler-specific override.
"""
import hashlib
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from conditional import (ClassDef, Conv, Ctor, Model, Operand, Reject, Result,  # noqa: E402
                         Type, conditional, cvset, LV, XV, PRV)

C = cvset("const")
T = Type


def cls(n, cv=frozenset()):
    return T("class", n, cv)


def A(n, cv=frozenset()):
    return T("arith", n, cv)


def ptr(t, cv=frozenset()):
    return T("ptr", inner=t, cv=cv)


cases = []   # (test, assertion, model, x, y, expected, expected rule or None)


def case(test, what, m, x, y, exp, rule=None):
    cases.append((test, what, m, x, y, exp, rule))


O = Operand
# ---------------------------------------------------------------- class tests
m = Model([ClassDef("Wide", convs=[Conv(A("int")), Conv(A("long"))])])
case("class-ambiguous-conversion", "Wide lvalue : short lvalue", m,
     O(cls("Wide"), LV), O(A("short"), LV), "reject", "COND-16")
case("class-converts-best-conversion", "Wide lvalue : int lvalue", m,
     O(cls("Wide"), LV), O(A("int"), LV), "prvalue int")

m = Model([ClassDef("A", ctors=[Ctor(cls("B", C), "&")]),
           ClassDef("B", ctors=[Ctor(cls("A", C), "&")])])
case("class-ambiguous", "A{} : B{}", m, O(cls("A")), O(cls("B")), "reject", "COND-5")

m = Model([ClassDef("Base"),
           ClassDef("Derived", bases=[("Base", "public", False)],
                    ctors=[Ctor(cls("Base", C), "&")])])
case("class-base-const-derived-prvalue-rejected", "Base prvalue : const Derived prvalue", m,
     O(cls("Base")), O(cls("Derived", C)), "reject", "COND-5")
case("class-base-does-not-convert-to-derived", "Base prvalue : Derived prvalue", m,
     O(cls("Base")), O(cls("Derived")), "prvalue Base")

m = Model([ClassDef("Base", ctors=[Ctor(cls("Derived", C), "&")]),
           ClassDef("Derived", bases=[("Base", "public", False)])])
case("class-base-less-qualified-rejected", "Base prvalue : const Derived lvalue", m,
     O(cls("Base")), O(cls("Derived", C), LV), "reject", "COND-5")

m = Model([ClassDef("Base"), ClassDef("Derived", bases=[("Base", "public", False)])])
case("class-base-target-keeps-cv", "Derived lvalue : Base prvalue", m,
     O(cls("Derived"), LV), O(cls("Base")), "prvalue Base")
case("class-base-target-keeps-cv", "const Derived lvalue : const Base prvalue", m,
     O(cls("Derived", C), LV), O(cls("Base", C)), "prvalue const Base")
case("class-derived-prvalue-against-base-xvalue", "Derived prvalue : Base xvalue", m,
     O(cls("Derived")), O(cls("Base"), XV), "prvalue Base")
case("derived-base-lvalue", "D lvalue : B lvalue", m,
     O(cls("Derived"), LV), O(cls("Base"), LV), "lvalue Base")

m = Model([ClassDef("Box")])
case("class-const-and-volatile-lvalues", "const Box lvalue : volatile Box lvalue", m,
     O(cls("Box", C), LV), O(cls("Box", cvset("volatile")), LV), "reject", "COND-5")
case("class-same-type-toward-more-qualified", "const Box lvalue : Box prvalue", m,
     O(cls("Box", C), LV), O(cls("Box")), "prvalue const Box")
case("class-same-type-toward-more-qualified", "Box prvalue : const Box lvalue", m,
     O(cls("Box")), O(cls("Box", C), LV), "prvalue const Box")

m = Model([ClassDef("View", convs=[Conv(ptr(A("int", C)), cv=C)])])
case("class-converts-to-decayed-array", "View lvalue : int[3] lvalue", m,
     O(cls("View"), LV), O(T("array", inner=A("int"), bound=3), LV), "prvalue const int*")

m = Model([ClassDef("S", convs=[Conv(A("int"), cv=C)])])
case("class-converts-to-int", "S lvalue : 7", m, O(cls("S"), LV), O(A("int")), "prvalue int")

m = Model([ClassDef("B"),
           ClassDef("ToRef", convs=[Conv(cls("B"), "&")]),
           ClassDef("ToValue", convs=[Conv(cls("B"))]),
           ClassDef("ToRvalueRef", convs=[Conv(cls("B"), "&&")])])
case("class-converts-to-reference", "ToRef : B lvalue", m,
     O(cls("ToRef"), LV), O(cls("B"), LV), "lvalue B")
case("class-converts-to-reference", "ToValue : B lvalue", m,
     O(cls("ToValue"), LV), O(cls("B"), LV), "prvalue B")
case("class-converts-to-reference", "ToRvalueRef : B xvalue", m,
     O(cls("ToRvalueRef"), LV), O(cls("B"), XV), "xvalue B")
case("class-converts-to-reference", "ToRef : B xvalue", m,
     O(cls("ToRef"), LV), O(cls("B"), XV), "prvalue B")
case("class-converts-to-reference", "ToRvalueRef : B lvalue", m,
     O(cls("ToRvalueRef"), LV), O(cls("B"), LV), "prvalue B")

m = Model([ClassDef("Tag"),
           ClassDef("Wrap", ctors=[Ctor(cls("Tag", C), "&")],
                    convs=[Conv(cls("Tag"), cv=C, deleted=True)])])
case("class-deleted-conversion-counts", "Wrap lvalue : Tag lvalue", m,
     O(cls("Wrap"), LV), O(cls("Tag"), LV), "reject", "COND-5")
m = Model([ClassDef("Tag"),
           ClassDef("Wrap", ctors=[Ctor(cls("Tag", C), "&")],
                    convs=[Conv(cls("Tag"), cv=C, access="private")])])
case("class-private-conversion-counts", "Wrap lvalue : Tag lvalue", m,
     O(cls("Wrap"), LV), O(cls("Tag"), LV), "reject", "COND-5")

m = Model([ClassDef("S", copy="deleted", move="absent")])
case("class-lvalue-prvalue-copy", "S lvalue : S prvalue", m,
     O(cls("S"), LV), O(cls("S")), "reject", "COND-8")
m = Model([ClassDef("S", copy="private", move="ok")])
case("class-prvalue-lvalue-private-copy", "S prvalue : S lvalue", m,
     O(cls("S")), O(cls("S"), LV), "reject", "COND-8")
m = Model([ClassDef("S", copy="deleted", move="deleted")])
case("prvalue-no-copy", "S prvalue : S prvalue", m, O(cls("S")), O(cls("S")), "prvalue S")
case("prvalue-no-copy-argument", "S(1) : S(2)", m, O(cls("S")), O(cls("S")), "prvalue S")

m = Model([ClassDef("A"), ClassDef("B", ctors=[Ctor(cls("A"))])])
case("class-one-way", "A{} : B{A{}}", m, O(cls("A")), O(cls("B")), "prvalue B")
m = Model([ClassDef("A"), ClassDef("B")])
case("no-common-type", "A{} : B{}", m, O(cls("A")), O(cls("B")), "reject", "COND-5")

m = Model([ClassDef("Meter", ctors=[Ctor(A("int"))], convs=[Conv(A("int"), cv=C)])])
case("nonclass-and-class-both-ways", "Meter lvalue : 1", m,
     O(cls("Meter"), LV), O(A("int")), "reject", "COND-5")
m = Model([ClassDef("Meter", ctors=[Ctor(A("int"))])])
case("nonclass-converts-to-class", "Meter lvalue : 1", m,
     O(cls("Meter"), LV), O(A("int")), "prvalue Meter")
case("nonclass-converts-to-class", "1 : Meter lvalue", m,
     O(A("int")), O(cls("Meter"), LV), "prvalue Meter")

m = Model([ClassDef("A", convs=[Conv(A("int"), cv=C)]), ClassDef("B", convs=[Conv(A("int"), cv=C)])])
case("two-classes-convert-to-int", "A lvalue : B lvalue", m,
     O(cls("A"), LV), O(cls("B"), LV), "prvalue int")
m = Model([ClassDef("A", convs=[Conv(ptr(A("int")), cv=C)]), ClassDef("B", convs=[Conv(ptr(A("int")), cv=C)])])
case("two-classes-convert-to-pointer", "A lvalue : B lvalue", m,
     O(cls("A"), LV), O(cls("B"), LV), "prvalue int*")
m = Model([ClassDef("First", convs=[Conv(A("float"))]), ClassDef("Second", convs=[Conv(A("float"))])])
case("two-classes-float", "First lvalue : Second lvalue", m,
     O(cls("First"), LV), O(cls("Second"), LV), "prvalue float")
m = Model([ClassDef("ToInt", convs=[Conv(A("int"))]), ClassDef("ToPointer", convs=[Conv(ptr(A("int")))])])
case("two-classes-no-candidate", "ToInt lvalue : ToPointer lvalue", m,
     O(cls("ToInt"), LV), O(cls("ToPointer"), LV), "reject", "COND-5")

COLOR = T("enum", "Color")
MP = T("mptr", "B", inner=A("int"))
to = [("ch1", A("char")), ("ch2", A("char")), ("b1", A("bool")), ("b2", A("bool")),
      ("e1", COLOR), ("e2", COLOR), ("i1", A("int")), ("d1", A("double")),
      ("l1", A("long")), ("p1", ptr(A("int"))), ("p2", ptr(A("int", C))),
      ("m1", MP), ("m2", MP)]
m = Model([ClassDef("B")] + [ClassDef(n, convs=[Conv(t)]) for n, t in to], enums={"Color": False})
for a, b, exp in [("ch1", "ch2", "prvalue int"), ("b1", "b2", "prvalue int"),
                  ("e1", "e2", "prvalue int"), ("i1", "d1", "prvalue double"),
                  ("i1", "l1", "prvalue long"), ("p1", "p2", "prvalue const int*"),
                  ("m1", "m2", "prvalue int B::*")]:
    case("two-classes-promoted-types", f"{a} : {b}", m, O(cls(a), LV), O(cls(b), LV), exp)

MODE = T("enum", "Mode", scoped=True)
m = Model([ClassDef("First", convs=[Conv(MODE)]), ClassDef("Second", convs=[Conv(MODE)])],
          enums={"Mode": True})
case("two-classes-scoped-enumeration", "First lvalue : Second lvalue", m,
     O(cls("First"), LV), O(cls("Second"), LV), "prvalue Mode")
m = Model([ClassDef("Wide", convs=[Conv(A("int")), Conv(A("long"))]),
           ClassDef("Narrow", convs=[Conv(A("int"))])])
case("two-classes-several-conversions", "Wide lvalue : Narrow lvalue", m,
     O(cls("Wide"), LV), O(cls("Narrow"), LV), "reject", "COND-5")

# ---------------------------------------------------------------- non-class rows
E = T("enum", "E")
F = T("enum", "F")
EM = T("enum", "Mode", scoped=True)
m = Model([ClassDef("S")], enums={"E": False, "F": False, "Mode": True})
i, l, d = A("int"), A("long"), A("double")
nc = [
    ("arithmetic-prvalue", "i : l", O(i, LV), O(l, LV), "prvalue long"),
    ("arithmetic-prvalue", "i : d", O(i, LV), O(d, LV), "prvalue double"),
    ("arithmetic-prvalue", "'a' : 'b'", O(A("char")), O(A("char")), "prvalue char"),
    ("arithmetic-prvalue", "(char)1 : (short)2", O(A("char")), O(A("short")), "prvalue int"),
    ("array-function-operands", "three : four", O(T("array", inner=i, bound=3), LV),
     O(T("array", inner=i, bound=4), LV), "prvalue int*"),
    ("array-function-operands", "\"ab\" : \"xyz\"", O(T("array", inner=A("char", C), bound=3), LV),
     O(T("array", inner=A("char", C), bound=4), LV), "prvalue const char*"),
    ("array-function-operands", "\"ab\" : \"cd\"", O(T("array", inner=A("char", C), bound=3), LV),
     O(T("array", inner=A("char", C), bound=3), LV), "lvalue const char[3]"),
    ("bitfield-or-enum", "e : e", O(E, LV), O(E, LV), "lvalue E"),
    ("bitfield-or-enum", "A : B", O(E), O(E), "prvalue E"),
    ("cv-lvalues-and-strings", "x : cx", O(i, LV), O(A("int", C), LV), "lvalue const int"),
    ("cv-lvalues-and-strings", "\"ab\" : \"c\"", O(T("array", inner=A("char", C), bound=3), LV),
     O(T("array", inner=A("char", C), bound=2), LV), "prvalue const char*"),
    ("lvalue-result", "x : y", O(i, LV), O(i, LV), "lvalue int"),
    ("member-pointer-nullptr", "&S::v : nullptr", O(T("mptr", "S", inner=i)), O(T("nullptr")),
     "prvalue int S::*"),
    ("mixed-enum-types", "A : C", O(E), O(F), "prvalue int"),
    ("pointer-and-int", "p : 1", O(ptr(i), LV), O(i), "reject", "COND-5"),
    ("pointer-operands", "&x : nullptr", O(ptr(i)), O(T("nullptr")), "prvalue int*"),
    ("pointer-operands", "&x : &cx", O(ptr(i)), O(ptr(A("int", C))), "prvalue const int*"),
    ("pointer-operands", "nullptr : nullptr", O(T("nullptr")), O(T("nullptr")), "prvalue std::nullptr_t"),
    ("same-type-prvalue", "letter : 'b'", O(A("char"), LV), O(A("char")), "prvalue char"),
    ("same-type-prvalue", "fixed : 'c'", O(A("char", C), LV), O(A("char")), "prvalue char"),
    ("same-type-prvalue", "count : (short)2", O(A("short"), LV), O(A("short")), "prvalue short"),
    ("same-type-prvalue", "true : false", O(A("bool")), O(A("bool")), "prvalue bool"),
    ("same-type-prvalue", "flag : 1", O(A("bool"), LV), O(i), "prvalue int"),
    ("same-type-prvalue", "Mode::on : Mode::off", O(EM), O(EM), "prvalue Mode"),
    ("same-type-prvalue", "red : green", O(E), O(E), "prvalue E"),
    ("same-type-prvalue", "1.0f : 2.0f", O(A("float")), O(A("float")), "prvalue float"),
    ("same-type-prvalue", "(char)1 : (unsigned char)2", O(A("char")), O(A("unsigned char")), "prvalue int"),
    ("two-zeros", "0 : 0", O(i, null_constant=True), O(i, null_constant=True), "prvalue int"),
    ("two-zeros", "nullptr : nullptr", O(T("nullptr")), O(T("nullptr")), "prvalue std::nullptr_t"),
    ("zero-and-nullptr", "0 : nullptr", O(i, null_constant=True), O(T("nullptr")), "prvalue std::nullptr_t"),
    ("zero-and-nullptr", "nullptr : 0", O(T("nullptr")), O(i, null_constant=True), "prvalue std::nullptr_t"),
    ("xvalue-operands", "int xvalue : int xvalue", O(i, XV), O(i, XV), "xvalue int"),
    ("xvalue-operands", "a : int xvalue", O(i, LV), O(i, XV), "prvalue int"),
    ("throw-operand", "x : throw 1", O(i, LV), O(i, throw=True), "lvalue int"),
    ("throw-operand", "throw 1 : 2", O(i, throw=True), O(i), "prvalue int"),
    ("void-and-value", "f() : 1", O(T("void")), O(i), "reject", "COND-13"),
    ("value-and-void", "x : (void)x", O(i, LV), O(T("void")), "reject", "COND-13"),
    ("void-operands", "f() : g()", O(T("void")), O(T("void")), "prvalue void"),
    ("const-volatile-lvalues-prvalue", "c : v", O(A("int", C), LV), O(A("int", cvset("volatile")), LV),
     "prvalue int"),
    ("const-and-plain-lvalue", "c : m", O(A("int", C), LV), O(i, LV), "lvalue const int"),
    ("mixed-category-prvalue", "x : 2", O(i, LV), O(i), "prvalue int"),
]
for row in nc:
    test, what, x, y, exp = row[:5]
    rule = row[5] if len(row) > 5 else None
    case(test, what, m, x, y, exp, rule)


def outcome(r):
    if isinstance(r, Reject):
        return "reject"
    return f"{r.cat} {r.type}"


def main():
    sha = hashlib.sha1(open(os.path.join(HERE, "conditional.py"), "rb").read()).hexdigest()
    frozen = open(os.path.join(HERE, "conditional.sha1")).read().split()[0]
    print(f"conditional.py sha1 {sha} ({'matches' if sha == frozen else 'DIFFERS FROM'} the Phase 1 record)")
    right = wrong = rule_diff = 0
    for test, what, m, x, y, exp, rule in cases:
        try:
            r = conditional(x, y, m)
            got = outcome(r)
        except Exception as e:  # a crash counts as wrong
            r, got = None, f"crash: {type(e).__name__}: {e}"
        ok = got == exp
        right += ok
        wrong += not ok
        extra = ""
        if ok and rule and isinstance(r, Reject) and r.rule != rule:
            rule_diff += 1
            extra = f"   [rule: component {r.rule}, test cites {rule}]"
        mark = "ok  " if ok else "FAIL"
        detail = str(r) if r is not None else got
        print(f"{mark} {test}: {what}\n       expected {exp}; got {detail}{extra}")
    print(f"\n{right} of {right + wrong} assertions right, {wrong} wrong; "
          f"{rule_diff} right rejections that cite a different rule")


if __name__ == "__main__":
    main()
