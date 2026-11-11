#!/usr/bin/env python3
"""Blind-phase checks of ctad.py against the examples on the CTAD page itself (no tests read)."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ctad import *
from ctad import D

T, U = TParam("T"), TParam("U")
INT_, LONG, SHORT, CHAR, DOUBLE = Prim("int"), Prim("long"), Prim("short"), Prim("char"), Prim("double")


def S(name, *args):
    return Cls(name, tuple(args), is_template=True)


def lv(t):
    return Arg(t, "lvalue")


def pr(t):
    return Arg(t, "prvalue")


results = []


def check(label, got, want):
    ok = (want == "fail" and got.kind == "fail") or (isinstance(want, str) and want.startswith("CTAD") and got.kind == "fail" and got.rule == want) \
        or (got.kind == "deduced" and got.type == want)
    results.append(ok)
    print(("ok  " if ok else "BAD ") + label + ": " + repr(got))


# example 1
w = World()
w.templates["Box"] = ClassTemplate("Box", [TP("T")], ctors=[Ctor([FP(T)])])
check("Box a(1)", ctad("Box", Init("paren", [pr(INT_)]), w), S("Box", INT_))
check("Box b = a", ctad("Box", Init("copy", [lv(S("Box", INT_))]), w), S("Box", INT_))
w.guides.append(UserGuide("Box", [FP(INT_)], S("Box", LONG), pos=5))
check("Box c(1) after guide", ctad("Box", Init("paren", [pr(INT_)], use_pos=6), w), S("Box", LONG))
check("Box e(n) short", ctad("Box", Init("paren", [lv(SHORT)], use_pos=6), w), S("Box", SHORT))
w.templates["Pair"] = ClassTemplate("Pair", [TP("T"), TP("U")], aggregate=True,
                                    elements=[Element(T, name="first"), Element(U, name="second")])
check("Pair p{1,'c'}", ctad("Pair", Init("list", [pr(INT_), pr(CHAR)]), w), S("Pair", INT_, CHAR))
w.aliases["Same"] = AliasTemplate("Same", [TP("V")], S("Pair", TParam("V"), TParam("V")))
check("Same s{1,2}", ctad("Same", Init("list", [pr(INT_), pr(INT_)]), w), S("Pair", INT_, INT_))
check("Same t{1,'c'}", ctad("Same", Init("list", [pr(INT_), pr(CHAR)]), w), "fail")
check("Box* q", check_place("pointer") or Result(None, ""), "CTAD-2")
B = TParam("B")
w.templates["Mix"] = ClassTemplate("Mix", [TP("B", pack=True)], aggregate=True,
                                   elements=[Element(Expand(B), "base"), Element(INT_, name="x")])
check("Mix m{5}", ctad("Mix", Init("list", [pr(INT_)]), w), S("Mix"))

# example 2: Chars and Bytes
N = CParam("N")
w.templates["Chars"] = ClassTemplate("Chars", [TP("N", "const", ctype=INT_)], aggregate=True,
                                     elements=[Element(Arr(CHAR, N), name="s")])
w.templates["Bytes"] = ClassTemplate("Bytes", [TP("N", "const", ctype=INT_)], aggregate=True,
                                     elements=[Element(Arr(Prim("unsigned char"), N), name="s")])
lit = lv(Arr(Prim("char", cvs(C)), 4))
check("Chars c{\"abc\"}", ctad("Chars", Init("list", [lit]), w), S("Chars", Val(4, INT_)))
check("Bytes u{\"abc\"}", ctad("Bytes", Init("list", [lit]), w), "fail")

# example 3: inherited guide gives no T
w.templates["Base"] = ClassTemplate("Base", [TP("T")], ctors=[Ctor([FP(T)])])
w.templates["Derived"] = ClassTemplate("Derived", [TP("T")], inherits=[S("Base", INT_)])
check("Derived d(1)", ctad("Derived", Init("paren", [pr(INT_)]), w), "fail")

# example 4: own versus inherited
w.templates["Own"] = ClassTemplate("Own", [TP("T")], ctors=[Ctor([FP(T)])], inherits=[S("Base", T)])
check("Own a(1)", ctad("Own", Init("paren", [pr(INT_)]), w), S("Own", INT_))
w.templates["Ref"] = ClassTemplate("Ref", [TP("T")], ctors=[Ctor([FP(LRef(Prim("T") if False else D.add_cv(T, cvs(C))))])],
                                   inherits=[S("Base", T)])
check("Ref b(1)", ctad("Ref", Init("paren", [pr(INT_)]), w), "CTAD-24")

# example 5: Size
w.templates["Size"] = ClassTemplate("Size", [TP("T")], ctors=[Ctor([FP(T)])])
check("new Size(2.5)", ctad("Size", Init("paren", [pr(DOUBLE)]), w), S("Size", DOUBLE))
check("Size x(1), y(2)", ctad_declarators("Size", [Init("paren", [pr(INT_)]), Init("paren", [pr(INT_)])], w), S("Size", INT_))
check("Size u(1), v(2.0)", ctad_declarators("Size", [Init("paren", [pr(INT_)]), Init("paren", [pr(DOUBLE)])], w), "CTAD-3")

# example 6
w.templates["Box2"] = ClassTemplate("Box2", [TP("T")], ctors=[Ctor([FP(U)], [TP("U")])])
check("Box two(one)", ctad("Box2", Init("paren", [lv(S("Box2", INT_))]), w), S("Box2", INT_))
check("Box three(3)", ctad("Box2", Init("paren", [pr(INT_)]), w), "CTAD-24")
w.templates["Range"] = ClassTemplate("Range", [TP("T")], ctors=[Ctor([FP(Ptr(T)), FP(Ptr(T))])])
check("Range r(data, data+3)", ctad("Range", Init("paren", [lv(Arr(INT_, 3)), pr(Ptr(INT_))]), w), S("Range", INT_))
w.templates["Tag"] = ClassTemplate("Tag", [TP("T", default=INT_)])
check("Tag none", ctad("Tag", Init("none", []), w), S("Tag", INT_))

# example 7: IntDuo
w.templates["Duo"] = ClassTemplate("Duo", [TP("T"), TP("U")], ctors=[Ctor([FP(T), FP(U)])])
w.aliases["IntDuo"] = AliasTemplate("IntDuo", [TP("W")], S("Duo", INT_, TParam("W")))
check("IntDuo d(1L,'c')", ctad("IntDuo", Init("paren", [pr(LONG), pr(CHAR)]), w), S("Duo", INT_, CHAR))

# example 8: Mark, Text, Wrap, Bases
w.templates["Mark"] = ClassTemplate("Mark", [TP("T")], ctors=[
    Ctor([FP(T)]), Ctor([FP(T), FP(Ptr(U), default=True)], [TP("U", default=INT_)])])
check("Mark m(1)", ctad("Mark", Init("paren", [pr(INT_)]), w), S("Mark", INT_))
w.templates["Text"] = ClassTemplate("Text", [TP("T")], ctors=[Ctor([FP(U)], [TP("U")])])
w.guides.append(UserGuide("Text", [FP(RRef(T))], S("Text", T), [TP("T")], pos=1))
w.guides.append(UserGuide("Text", [FP(Ptr(Prim("char", cvs(C))))], S("Text", LONG), pos=2))
check("Text t(\"ab\")", ctad("Text", Init("paren", [lv(Arr(Prim("char", cvs(C)), 3))]), w), S("Text", LONG))
w.templates["Base3"] = ClassTemplate("Base3", [TP("T")], aggregate=True, elements=[Element(T, name="v")])
w.templates["Wrap"] = ClassTemplate("Wrap", [TP("T")], aggregate=True,
                                    elements=[Element(S("Base3", T), "base"), Element(T, name="w")])
check("Wrap x{{1},2}", ctad("Wrap", Init("list", [Braced([pr(INT_)]), pr(INT_)]), w), S("Wrap", INT_))
check("Wrap y{1,2}", ctad("Wrap", Init("list", [pr(INT_), pr(INT_)]), w), "fail")
X, Y, Z = Cls("X"), Cls("Y"), Cls("Z")
w.templates["List"] = ClassTemplate("List", [TP("T", pack=True)], aggregate=True)
w.templates["Bases"] = ClassTemplate("Bases", [TP("T", pack=True)], aggregate=True,
                                     elements=[Element(S("List", Expand(T)), "base"), Element(Expand(T), "base")])
check("Bases b{List<X,Y,Z>{}, X{}}", ctad("Bases", Init("list", [pr(S("List", X, Y, Z)), pr(X)]), w), S("Bases", X, Y, Z))
check("Bases c{List<X,Y,Z>{}, Y{}}", ctad("Bases", Init("list", [pr(S("List", X, Y, Z)), pr(Y)]), w), "fail")


# rules with no page example
w = World()
w.templates["E"] = ClassTemplate("E", [TP("T")], ctors=[Ctor([FP(T)], explicit=True)])
check("E e = 1 (CTAD-17)", ctad("E", Init("copy", [pr(INT_)]), w), "CTAD-24")
check("E e = {1} (CTAD-23)", ctad("E", Init("copy-list", [pr(INT_)]), w), "CTAD-23")
check("E e{1}", ctad("E", Init("list", [pr(INT_)]), w), S("E", INT_))
w.templates["F"] = ClassTemplate("F", [TP("T")], ctors=[Ctor([FP(RRef(T))])])
check("F f(lvalue) (CTAD-8)", ctad("F", Init("paren", [lv(INT_)]), w), "CTAD-24")
check("F f(1)", ctad("F", Init("paren", [pr(INT_)]), w), S("F", INT_))
w.templates["G"] = ClassTemplate("G", [TP("T")], ctors=[Ctor([FP(RRef(U))], [TP("U")])])
w.templates["Nd"] = ClassTemplate("Nd", [TP("T")], defined=False)
w.guides.append(UserGuide("Nd", [FP(INT_)], S("Nd", LONG)))
check("Nd n(1) (CTAD-6, CTAD-11)", ctad("Nd", Init("paren", [pr(INT_)]), w), S("Nd", LONG))
w.templates["Del"] = ClassTemplate("Del", [TP("T")], ctors=[Ctor([FP(T)], deleted=True)])
check("Del d(1) (CTAD-26)", ctad("Del", Init("paren", [pr(INT_)]), w), "CTAD-26")
w.templates["Con"] = ClassTemplate("Con", [TP("T")], ctors=[Ctor([FP(T)], constraint=lambda b: b["T"] == INT_)])
check("Con c(1.0) (CTAD-29)", ctad("Con", Init("paren", [pr(DOUBLE)]), w), "CTAD-24")
IL = lambda e: Cls("std::initializer_list", (e,), is_template=True)
w.templates["V"] = ClassTemplate("V", [TP("T")], ctors=[Ctor([FP(IL(T))]), Ctor([FP(INT_), FP(T)])])
check("V v{1, 2.0}: phase 1 fails, phase 2", ctad("V", Init("list", [pr(INT_), pr(DOUBLE)]), w), S("V", DOUBLE))
check("V v{1, 2}: phase 1", ctad("V", Init("list", [pr(INT_), pr(INT_)]), w), S("V", INT_))
check("V v{V<int>} (CTAD-18)", ctad("V", Init("list", [lv(S("V", INT_))]), w), S("V", INT_))
check("CTAD-13", check_user_guide(UserGuide("V", [FP(INT_)], S("V", T), [TP("T")]), w) or Result(None, ""), "CTAD-13")
check("CTAD-12 cv", check_user_guide(UserGuide("V", [FP(INT_)], Cls("V", (INT_,), cvs(C), True)), w) or Result(None, ""), "CTAD-12")
check("CTAD-15", ctad("P", Init("paren", [pr(INT_)]), World(aliases={"P": AliasTemplate("P", [TP("T")], Ptr(T), other_type=True)})), "CTAD-15")

print(f"{sum(results)} of {len(results)}")
