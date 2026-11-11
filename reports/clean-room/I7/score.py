#!/usr/bin/env python3
"""Score the frozen ctad.py on the tests that cite the CTAD rules.
Each assertion is (test, label, expected, thunk). expected is a type string, 'accept'
(the call gives no failure), or 'reject' (the call fails). A rule after 'reject:'
is compared for the rule column only."""
import hashlib, os, sys
here = os.path.dirname(os.path.abspath(__file__))
want_sha = open(os.path.join(here, "ctad.sha1")).read().split()[0]
got_sha = hashlib.sha1(open(os.path.join(here, "ctad.py"), "rb").read()).hexdigest()
if want_sha != got_sha:
    sys.exit(f"ctad.py changed: {got_sha} is not the frozen {want_sha}")
sys.path.insert(0, here)
from ctad import *
from ctad import D

T, U, V_, A_, B_ = TParam("T"), TParam("U"), TParam("V"), TParam("A"), TParam("B")
I = Prim("int"); L = Prim("long"); SH = Prim("short"); CH = Prim("char"); DB = Prim("double")
UN = Prim("unsigned int"); LL = Prim("long long")
CCH = Prim("char", cvs(C))
SIZES = {"char": 1, "short": 2, "int": 4, "long": 8, "double": 8, "unsigned int": 4, "long long": 8}


def S(n, *a):
    return Cls(n, tuple(a), is_template=True)


def lv(t):
    return Arg(t, "lvalue")


def pr(t):
    return Arg(t, "prvalue")


def size(t):
    return SIZES.get(getattr(t, "name", ""), 8)


def lit(n):
    return lv(Arr(CCH, n))


def ct(name, tps, **kw):
    return ClassTemplate(name, tps, **kw)


tp = TP
A = []   # assertions


def add(test, label, expected, thunk, note=""):
    A.append((test, label, expected, thunk, note))


def run1(name, form, args, w, **kw):
    return lambda w=w: ctad(name, Init(form, args, **kw), w)


def Box1():
    return ct("Box", [tp("T")], ctors=[Ctor([FP(T)])])


# ------------------------------------------------------------------ the grid (forms.json)
IL = lambda e: S("std::initializer_list", e)
cols = {}
w = World(); w.templates["C"] = ct("C", [tp("T")], ctors=[Ctor([FP(T)])]); cols["constructor"] = (w, "C", "C<int>")
w = World(); w.templates["C"] = ct("C", [tp("T")], ctors=[Ctor([FP(T), FP(U, default=True)], [tp("U", default=I)])]); cols["constructor template"] = (w, "C", "C<int>")
w = World(); w.templates["C"] = ct("C", [tp("T")], ctors=[Ctor([FP(T)], explicit=True)]); cols["explicit constructor"] = (w, "C", "C<int>")
w = World(); w.templates["C"] = ct("C", [tp("T")], ctors=[Ctor([FP(IL(T))])]); cols["initializer-list constructor"] = (w, "C", "C<int>")
w = World(); w.templates["C"] = ct("C", [tp("T")], aggregate=True, elements=[Element(T, name="v")]); cols["aggregate"] = (w, "C", "C<int>")
w = World(); w.templates["C"] = ct("C", [tp("T")], ctors=[Ctor([FP(U)], [tp("U")])]); w.guides.append(UserGuide("C", [FP(U)], S("C", U), [tp("U")])); cols["user guide"] = (w, "C", "C<int>")
w = World(); w.templates["C"] = ct("C", [tp("T")], ctors=[Ctor([FP(U)], [tp("U")])]); w.guides.append(UserGuide("C", [FP(U)], S("C", U), [tp("U")], explicit=True)); cols["explicit user guide"] = (w, "C", "C<int>")
w = World(); w.templates["D"] = ct("D", [tp("T")], ctors=[Ctor([FP(T)])]); w.aliases["C"] = AliasTemplate("C", [tp("T")], S("D", T)); cols["alias template"] = (w, "C", "D<int>")
w = World(); w.templates["B"] = ct("B", [tp("T")], ctors=[Ctor([FP(T)])]); w.templates["C"] = ct("C", [tp("T")], inherits=[S("B", T)]); cols["inherited constructor"] = (w, "C", "C<int>")
rows = {"parentheses": ("paren", False), "braces": ("list", False), "copy": ("copy", False), "copy-list": ("copy-list", False),
        "functional cast": ("paren", False), "braced cast": ("list", False), "new": ("paren", False),
        "copy, parentheses": ("paren", True), "copy, braces": ("list", True), "copy, equals": ("copy", True)}
reject_cells = {("parentheses", "initializer-list constructor"), ("copy", "explicit constructor"),
                ("copy", "initializer-list constructor"), ("copy", "aggregate"), ("copy", "explicit user guide"),
                ("copy-list", "explicit constructor"), ("copy-list", "explicit user guide"),
                ("functional cast", "initializer-list constructor"), ("new", "initializer-list constructor")}
for r, (form, copy) in rows.items():
    for c, (w, name, want) in cols.items():
        exp = "reject" if (r, c) in reject_cells else want
        arg = lv(Cls(want.split("<")[0], (I,), cvs(C), True)) if copy else pr(I)
        add("grids/forms.json", f"{r} / {c}", exp, run1(name, form, [arg], w))

# ------------------------------------------------------------------ aggregate tests
w = World()
w.templates["Base"] = ct("Base", [tp("T")], aggregate=True, elements=[Element(T, name="v")])
w.templates["Wrap"] = ct("Wrap", [tp("T")], aggregate=True, elements=[Element(S("Base", T), "base"), Element(T, name="w")])
add("aggregate-base", "Wrap x{{1}, 2}", "Wrap<int>", run1("Wrap", "list", [Braced([pr(I)]), pr(I)], w))
add("aggregate-base-no-elision", "Wrap y{1, 2}", "reject:CTAD-24", run1("Wrap", "list", [pr(I), pr(I)], w))

w = World()
N = CParam("N")
w.templates["Array"] = ct("Array", [tp("T"), tp("N", "const", ctype=I)], aggregate=True, elements=[Element(Arr(T, N), name="values")])
add("aggregate-candidate-arrays", "Array numbers{{1,2,3}}", "Array<int, 3>", run1("Array", "list", [Braced([pr(I)] * 3)], w))
add("aggregate-candidate-arrays", "Array text{\"abc\"}", "Array<char, 4>", run1("Array", "list", [lit(4)], w))

w = World()
w.templates["Pair"] = ct("Pair", [tp("T"), tp("U")], aggregate=True, elements=[Element(T, name="first"), Element(U, name="second")])
w.templates["Twice"] = ct("Twice", [tp("T")], aggregate=True, elements=[Element(T, name="x"), Element(T, name="y")])
add("aggregate-candidate", "Pair a{1,'c'}", "Pair<int, char>", run1("Pair", "list", [pr(I), pr(CH)], w))
add("aggregate-candidate", "Pair b(1, 2.0)", "Pair<int, double>", run1("Pair", "paren", [pr(I), pr(DB)], w))
add("aggregate-candidate", "Pair c{.first=1L,.second=2u}", "Pair<long, unsigned int>",
    run1("Pair", "list", [pr(L), pr(UN)], w, designators=["first", "second"]))
add("aggregate-candidate", "Twice t{1,2}", "Twice<int>", run1("Twice", "list", [pr(I), pr(I)], w))
add("aggregate-conflict", "Twice t{1, 2L}", "reject:CTAD-24", run1("Twice", "list", [pr(I), pr(L)], w))

w = World()
F = TParam("F")
w.templates["Overloads"] = ct("Overloads", [tp("F", pack=True)], aggregate=True, elements=[Element(Expand(F), "base")])
add("aggregate-candidate-pack", "Overloads all{ByInt{}, ByDouble{}}", "Overloads<ByInt, ByDouble>",
    run1("Overloads", "list", [pr(Cls("ByInt")), pr(Cls("ByDouble"))], w))

w = World()
w.templates["Row"] = ct("Row", [tp("N", "const", ctype=I)], aggregate=True, elements=[Element(Arr(I, N), name="v")])
add("aggregate-dependent-bound", "Row r{1,2,3}", "reject:CTAD-24", run1("Row", "list", [pr(I)] * 3, w))

w = World()
w.templates["Three"] = ct("Three", [tp("T")], aggregate=True, elements=[Element(Arr(T, 3), name="v")])
w.templates["Mixed"] = ct("Mixed", [tp("T")], aggregate=True, elements=[Element(Arr(T, 3), name="v"), Element(I, name="n")])
add("aggregate-dependent-element", "Three numbers{1,2,3}", "Three<int>", run1("Three", "list", [pr(I)] * 3, w))
add("aggregate-dependent-element", "Three words{\"ab\"}", "Three<const char*>", run1("Three", "list", [lit(3)], w))
add("aggregate-dependent-element", "Mixed m{1,2,3,4}", "Mixed<int>", run1("Mixed", "list", [pr(I)] * 4, w))

w = World()
w.templates["In"] = ct("In", [tp("T")], aggregate=True, elements=[Element(T, name="x"), Element(T, name="y")])
w.templates["Outer"] = ct("Outer", [tp("T")], aggregate=True, elements=[Element(S("In", T), name="in")])
w.templates["Mixed"] = ct("Mixed", [tp("T")], aggregate=True, elements=[Element(Cls("Plain"), name="p"), Element(T, name="z")])
add("aggregate-elision", "Outer a{In<int>{1,2}}", "Outer<int>", run1("Outer", "list", [pr(S("In", I))], w))
add("aggregate-elision", "Mixed b{1,2,3L}", "Mixed<long>", run1("Mixed", "list", [pr(I), pr(I), pr(L)], w))
add("aggregate-no-elision", "Outer o{1,2}", "reject:CTAD-24", run1("Outer", "list", [pr(I), pr(I)], w))

w = World()
w.templates["Agg"] = ct("Agg", [tp("T", default=I)], aggregate=True, elements=[Element(T, name="v")])
add("aggregate-empty-list", "Agg a{}", "Agg<int>", run1("Agg", "list", [], w))
w = World()
w.templates["Agg"] = ct("Agg", [tp("T")], aggregate=True, elements=[Element(T, name="v")])
add("aggregate-new-parentheses", "new Agg(1)", "Agg<int>", run1("Agg", "paren", [pr(I)], w))
add("aggregate-too-many", "Agg a{1,2}", "reject:CTAD-24", run1("Agg", "list", [pr(I), pr(I)], w))
w2 = World(templates=dict(w.templates))
w2.guides.append(UserGuide("Agg", [FP(Ptr(T))], S("Agg", T), [tp("T")]))
add("aggregate-user-guide", "Agg a{1} with a user guide", "reject:CTAD-24", run1("Agg", "list", [pr(I)], w2))

X, Y, Z = Cls("X"), Cls("Y"), Cls("Z")


def bases_world():
    w = World()
    w.templates["List"] = ct("List", [tp("T", pack=True)], aggregate=True)
    w.templates["Bases"] = ct("Bases", [tp("T", pack=True)], aggregate=True,
                              elements=[Element(S("List", Expand(T)), "base"), Element(Expand(T), "base")])
    return w


w = bases_world()
w.env.classes["ToY"] = ClassDef()
LXYZ = pr(S("List", X, Y, Z))
add("aggregate-pack-length-conflict", "Bases b{List<X,Y,Z>{}, {}, ToY{}}", "reject:CTAD-24",
    run1("Bases", "list", [LXYZ, Braced([]), pr(Cls("ToY"))], w))
add("aggregate-pack-length-fewer", "Bases b{List<X,Y,Z>{}, X{}}", "Bases<X, Y, Z>", run1("Bases", "list", [LXYZ, pr(X)], w))
add("aggregate-pack-length-fixed", "Bases b{List<X,Y,Z>{}, {}, {}}", "Bases<X, Y, Z>", run1("Bases", "list", [LXYZ, Braced([]), Braced([])], w))
add("aggregate-pack-length-parentheses", "Bases b(List<X,Y,Z>{}, X{})", "Bases<X, Y, Z>", run1("Bases", "paren", [LXYZ, pr(X)], w))
add("aggregate-pack-length-position", "Bases b{List<X,Y,Z>{}, Y{}}", "reject:CTAD-24", run1("Bases", "list", [LXYZ, pr(Y)], w))

w = World()
Bp = TParam("B")
w.templates["Tail"] = ct("Tail", [tp("B", pack=True)], aggregate=True, elements=[Element(Expand(Bp), "base"), Element(I, name="x")])
add("aggregate-pack-not-last", "Tail t{5}", "Tail<>", run1("Tail", "list", [pr(I)], w))
add("aggregate-pack-not-last-element", "Tail t{Empty{}, 5}", "reject:CTAD-24", run1("Tail", "list", [pr(Cls("Empty")), pr(I)], w))
w.templates["Both"] = ct("Both", [tp("F", pack=True)], aggregate=True, elements=[Element(Expand(F), "base")])
add("aggregate-pack-parentheses", "Both both(Left{}, Right{})", "Both<Left, Right>", run1("Both", "paren", [pr(Cls("Left")), pr(Cls("Right"))], w))

w = World()
w.templates["Bytes"] = ct("Bytes", [tp("N", "const", ctype=I)], aggregate=True, elements=[Element(Arr(Prim("unsigned char"), N), name="s")])
add("aggregate-string-element-type", "Bytes u{\"abc\"}", "reject:CTAD-24", run1("Bytes", "list", [lit(4)], w))
w.templates["Named"] = ct("Named", [tp("T")], aggregate=True, elements=[Element(Arr(CH, 3), name="v"), Element(T, name="n")])
add("aggregate-string-fixed-array", "Named b{\"ab\", 1}", "Named<int>", run1("Named", "list", [lit(3), pr(I)], w))

# ------------------------------------------------------------------ alias tests
w = World()
w.templates["Pair"] = ct("Pair", [tp("T"), tp("U")], ctors=[Ctor([FP(T), FP(U)])])
w.aliases["Same"] = AliasTemplate("Same", [tp("T")], S("Pair", T, T))
add("alias-template-conflict", "Same s(1, 'c')", "reject:CTAD-24", run1("Same", "paren", [pr(I), pr(CH)], w))
add("alias-template", "Same s(1, 2)", "Pair<int, int>", run1("Same", "paren", [pr(I), pr(I)], w))

INTEGRALS = {"int", "char", "short", "long", "unsigned int", "long long"}
w = World()
w.templates["Pair"] = ct("Pair", [tp("A"), tp("B")], ctors=[Ctor([FP(A_), FP(B_)])])
w.aliases["Plain"] = AliasTemplate("Plain", [tp("T")], S("Pair", Ptr(T), Ptr(T)))
w.aliases["Indexed"] = AliasTemplate("Indexed", [tp("T")], S("Pair", Ptr(T), Ptr(T)),
                                     constraint=lambda b: getattr(b["T"], "name", "") in INTEGRALS)
add("alias-template-constraints", "Plain p(&d, &d)", "Pair<double*, double*>", run1("Plain", "paren", [pr(Ptr(DB)), pr(Ptr(DB))], w))
add("alias-template-constraints", "Indexed q(&i, &i)", "Pair<int*, int*>", run1("Indexed", "paren", [pr(Ptr(I)), pr(Ptr(I))], w))
add("alias-template-constraints", "Indexed r(&d, &d)", "reject:CTAD-24", run1("Indexed", "paren", [pr(Ptr(DB)), pr(Ptr(DB))], w))

w = World()
w.aliases["A"] = AliasTemplate("A", [tp("T")], S("In", T), dependent_qualifier=True)
add("alias-template-dependent-qualifier", "A a(1)", "reject:CTAD-15", run1("A", "paren", [pr(I)], w), "caller marks the qualifier")

w = World()
w.templates["Pair"] = ct("Pair", [tp("T"), tp("U")], ctors=[Ctor([FP(T), FP(U)])])
w.aliases["IntPair"] = AliasTemplate("IntPair", [tp("U")], S("Pair", I, U))
w.aliases["Both"] = AliasTemplate("Both", [tp("T"), tp("U")], S("Pair", T, U))
w.aliases["LongFirst"] = AliasTemplate("LongFirst", [tp("U")], S("Both", L, U))
add("alias-template-fixed-argument", "IntPair p(1L, 'c')", "Pair<int, char>", run1("IntPair", "paren", [pr(L), pr(CH)], w))
add("alias-template-fixed-argument", "LongFirst q(1, 'c')", "Pair<long, char>", run1("LongFirst", "paren", [pr(I), pr(CH)], w))
add("alias-template-fixed-argument-exact", "IntPair p(1, 'c')", "Pair<int, char>", run1("IntPair", "paren", [pr(I), pr(CH)], w))
add("alias-template-fixed-argument-exact", "LongFirst q(1L, 'c')", "Pair<long, char>", run1("LongFirst", "paren", [pr(L), pr(CH)], w))

w = World()
w.templates["Box"] = ct("Box", [tp("T")], ctors=[Ctor([FP(U)], [tp("U")])])
w.guides.append(UserGuide("Box", [FP(Ptr(T))], S("Box", T), [tp("T")]))
w.aliases["Alias"] = AliasTemplate("Alias", [tp("T")], S("Box", T))
w.templates["Agg"] = ct("Agg", [tp("T"), tp("U")], aggregate=True, elements=[Element(T, name="first"), Element(U, name="second")])
w.aliases["IntAgg"] = AliasTemplate("IntAgg", [tp("U")], S("Agg", I, U))
add("alias-template-guides", "Alias a(&n)", "Box<int>", run1("Alias", "paren", [pr(Ptr(I))], w))
add("alias-template-guides", "Alias b(one)", "Box<char>", run1("Alias", "paren", [lv(Cls("Box", (CH,), cvs(C), True))], w))
add("alias-template-guides", "IntAgg c{1, 'c'}", "Agg<int, char>", run1("IntAgg", "list", [pr(I), pr(CH)], w))

w = World()
w.aliases["Pointer"] = AliasTemplate("Pointer", [tp("T")], Ptr(S("Box", T)), other_type=True)
add("alias-template-not-class", "Pointer p = ...", "reject:CTAD-15", run1("Pointer", "copy", [pr(Ptr(S("Box", I)))], w), "caller marks the alias")

w = World()
w.templates["Pair"] = ct("Pair", [tp("A"), tp("B")], ctors=[Ctor([FP(A_), FP(B_)])])
w.env.class_templates["Id"] = lambda args: ClassDef(members={"type": (args[0], "public")})
w.guides.append(UserGuide("Pair", [FP(A_), FP(B_)], S("Pair", A_, D.Qualified(S("Id", B_), "type")), [tp("A"), tp("B")]))
w.aliases["Same"] = AliasTemplate("Same", [tp("T")], S("Pair", Ptr(T), Ptr(T)))
add("alias-template-result-check", "Pair plain(&i, 1L)", "Pair<int*, long>", run1("Pair", "paren", [pr(Ptr(I)), pr(L)], w))
add("alias-template-result-check", "Same s(&i, 1L)", "reject:CTAD-24", run1("Same", "paren", [pr(Ptr(I)), pr(L)], w))

w = World()
w.templates["Ref"] = ct("Ref", [tp("T")], ctors=[Ctor([FP(RRef(T))])])
w.aliases["Alias"] = AliasTemplate("Alias", [tp("T")], S("Ref", T))
add("alias-template-rvalue-reference", "Alias a(i)", "reject:CTAD-24", run1("Alias", "paren", [lv(I)], w))

w = World()
w.templates["Box"] = Box1()
w.guides.append(UserGuide("Box", [FP(Ptr(T))], S("Box", L), [tp("T")]))
w.aliases["PointerBox"] = AliasTemplate("PointerBox", [tp("T")], S("Box", Ptr(T)))
add("alias-template-unreachable", "PointerBox b(&n)", "Box<int*>", run1("PointerBox", "paren", [pr(Ptr(I))], w))
add("alias-template-unreachable", "Box b(&n) (no alias)", "Box<long>", run1("Box", "paren", [pr(Ptr(I))], w))

# ------------------------------------------------------------------ choice
w = World()
w.templates["Two"] = ct("Two", [tp("T")], ctors=[Ctor([FP(T), FP(I)]), Ctor([FP(I), FP(T)])])
add("ambiguous-guides", "Two t(1, 2)", "reject:CTAD-24", run1("Two", "paren", [pr(I), pr(I)], w))

w = World()
w.templates["Wrapper"] = ct("Wrapper", [tp("Item")], aggregate=True, elements=[Element(TParam("Item"), name="item")])
add("braced-cast-class-template-member", "Wrapper{v}, V = long", "Wrapper<long>", run1("Wrapper", "list", [lv(L)], w), "after substitution")
add("braced-cast-function-template-return", "Wrapper{v}, V = int", "Wrapper<int>", run1("Wrapper", "list", [lv(I)], w), "after substitution")
add("braced-cast-function-template-variable", "Wrapper{closure}", "Wrapper<closure>", run1("Wrapper", "list", [pr(Cls("closure"))], w), "after substitution")

w = World()
w.templates["Narrow"] = ct("Narrow", [tp("T")], ctors=[Ctor([FP(T)], explicit=lambda b: size(b["T"]) > 1)])
add("conditional-explicit-copy-list", "Narrow b = {1}", "reject:CTAD-23", run1("Narrow", "copy-list", [pr(I)], w))
add("conditional-explicit", "Narrow a = {'c'}", "Narrow<char>", run1("Narrow", "copy-list", [pr(CH)], w))
add("conditional-explicit", "Narrow b = 'c'", "Narrow<char>", run1("Narrow", "copy", [pr(CH)], w))
add("conditional-explicit", "Narrow c{1}", "Narrow<int>", run1("Narrow", "list", [pr(I)], w))

w = World()
w.templates["Size"] = Box1(); w.templates["Size"].name = "Size"
w.guides.append(UserGuide("Size", [FP(I)], S("Size", L)))
add("constant-parameter-guide", "Hold<3>", "Size<long>", run1("Size", "copy", [pr(I)], w))
add("constant-parameter-guide", "Hold<'a'>", "Size<char>", run1("Size", "copy", [pr(CH)], w))
add("constant-parameter-guide", "place: constant parameter", "accept", lambda: check_place("constant-param") or Result(None, ""))

w = World()
w.templates["Ptr"] = ct("Ptr", [tp("T")], ctors=[Ctor([FP(Ptr(T))]), Ctor([FP(Ptr(U)), FP(T)], [tp("U")])])
w.templates["Opt"] = ct("Opt", [tp("T")], ctors=[Ctor([FP(T), FP(U, default=True)], [tp("U", default=T)])])
w.templates["Tuple"] = ct("Tuple", [tp("T", pack=True)], ctors=[Ctor([FP(T, pack=True)])])
add("constructor-guides", "Ptr a(&n)", "Ptr<int>", run1("Ptr", "paren", [pr(Ptr(I))], w))
add("constructor-guides", "Ptr b(&n, 'c')", "Ptr<char>", run1("Ptr", "paren", [pr(Ptr(I)), pr(CH)], w))
add("constructor-guides", "Opt o(1)", "Opt<int>", run1("Opt", "paren", [pr(I)], w))
add("constructor-guides", "Tuple t(1, 'c', 2.0)", "Tuple<int, char, double>", run1("Tuple", "paren", [pr(I), pr(CH), pr(DB)], w))
add("constructor-guides", "Tuple e", "Tuple<>", run1("Tuple", "none", [], w))

w = World()
w.templates["Wrap"] = ct("Wrap", [tp("T")], ctors=[Ctor([FP(T)])])
add("constructor-member-alias", "Wrap w(1)", "Wrap<int>", run1("Wrap", "paren", [pr(I)], w), "caller resolves value_type to T")
w = World()
w.templates["Wrap"] = ct("Wrap", [tp("T")], ctors=[Ctor([FP(D.Qualified(S("Id", T), "type"))])])
add("constructor-nondeduced", "Wrap w(1)", "reject:CTAD-24", run1("Wrap", "paren", [pr(I)], w))

w = World()
w.templates["Hold"] = ct("Hold", [tp("T")], ctors=[Ctor([FP(RRef(T))])])
add("constructor-rvalue-reference", "Hold h(1)", "Hold<int>", run1("Hold", "paren", [pr(I)], w))
add("constructor-rvalue-reference-lvalue", "Hold h(n)", "reject:CTAD-24", run1("Hold", "paren", [lv(I)], w))
w = World()
w.templates["Hold"] = ct("Hold", [tp("T")], ctors=[Ctor([FP(RRef(T)), FP(RRef(U))], [tp("U")])])
add("constructor-template-forwarding", "Hold h(1, i)", "Hold<int>", run1("Hold", "paren", [pr(I), lv(I)], w))

w = World()
w.templates["Box"] = Box1()
one = lv(Cls("Box", (I,), cvs(C), True))
add("copy-candidate", "Box a(one)", "Box<int>", run1("Box", "paren", [one], w))
add("copy-candidate", "Box b = one", "Box<int>", run1("Box", "copy", [one], w))
add("copy-candidate", "Box c{one}", "Box<int>", run1("Box", "list", [one], w))
add("copy-candidate", "Box d = {one}", "Box<int>", run1("Box", "copy-list", [one], w))
w = World()
w.templates["Box"] = ct("Box", [tp("T")], ctors=[Ctor([FP(U)], [tp("U")])])
add("copy-candidate-only", "Box two(one)", "Box<int>", run1("Box", "paren", [one], w))

w = World()
w.templates["Only"] = ct("Only", [tp("T")], defined=False)
add("declared-not-defined-no-guide", "Only d", "reject:CTAD-24", run1("Only", "none", [], w))
w.guides.append(UserGuide("Only", [FP(I)], S("Only", I)))
add("declared-not-defined", "Only d(1)", "Only<int>", run1("Only", "paren", [pr(I)], w))

w = World()
w.templates["In"] = ct("In", [tp("U")], ctors=[Ctor([FP(U)])])
add("dependent-qualifier", "typename Outer<T>::In x(1)", "In<int>", run1("In", "paren", [pr(I)], w), "after substitution")
add("dependent-qualifier-template-keyword", "typename Outer<T>::template In x(1)", "In<int>", run1("In", "paren", [pr(I)], w), "after substitution")

w = World()
w.templates["Box"] = ct("Box", [tp("T", default=I)], ctors=[Ctor([FP(T)])])
add("empty-argument-list", "Box<> a(2): no deduction", "accept", lambda: check_place("variable", has_list=True) or Result(None, ""))
add("empty-argument-list", "Box b(2.5)", "Box<double>", run1("Box", "paren", [pr(DB)], w))

w = World()
w.templates["Strict"] = ct("Strict", [tp("T")], ctors=[Ctor([FP(T)], explicit=True)])
add("explicit-copy-list", "Strict s = {1}", "reject:CTAD-23", run1("Strict", "copy-list", [pr(I)], w))
add("selection-explicit-copy", "Strict s = 1", "reject:CTAD-24", run1("Strict", "copy", [pr(I)], w))

w = World()
w.templates["Box"] = ct("Box", [tp("T")], ctors=[Ctor([FP(U)], [tp("U")]), Ctor([FP(T)])])
w.guides.append(UserGuide("Box", [FP(Ptr(T))], S("Box", T), [tp("T")], explicit=True))
add("explicit-user-guide-copy", "Box a(&n)", "Box<int>", run1("Box", "paren", [pr(Ptr(I))], w))
add("explicit-user-guide-copy", "Box b = &n", "Box<int*>", run1("Box", "copy", [pr(Ptr(I))], w))
w = World()
w.templates["Box"] = Box1()
w.guides.append(UserGuide("Box", [FP(Ptr(T))], S("Box", T), [tp("T")], explicit=True))
add("explicit-user-guide-copy-list", "Box b = {&n}", "reject:CTAD-23", run1("Box", "copy-list", [pr(Ptr(I))], w))

# ------------------------------------------------------------------ user guide declarations
w = World()
G = lambda *a, **k: UserGuide("Box", *a, **k)
gA = G([FP(Ptr(T))], S("Box", T), [tp("T")])
add("guide-redeclared", "the same guide twice", "reject:CTAD-12", lambda w=w: check_user_guide(gA, w, [gA, G([FP(Ptr(T))], S("Box", T), [tp("T")])]) or Result(None, ""))
add("guide-result-not-specialization", "-> int", "reject:CTAD-12", lambda w=w: check_user_guide(G([FP(Ptr(T))], I, [tp("T")]), w) or Result(None, ""))
add("guide-result-other-template", "-> Other<T>", "reject:CTAD-12", lambda w=w: check_user_guide(G([FP(Ptr(T))], S("Other", T), [tp("T")]), w) or Result(None, ""))
add("guide-result-qualified", "-> const Box<T>", "reject:CTAD-12", lambda w=w: check_user_guide(G([FP(Ptr(T))], Cls("Box", (T,), cvs(C), True), [tp("T")]), w) or Result(None, ""))
g1, g2 = G([FP(I)], S("Box", L)), G([FP(I)], S("Box", SH))
add("guide-same-parameters", "Box(int) twice", "reject:CTAD-12", lambda w=w: check_user_guide(g2, w, [g1, g2]) or Result(None, ""))
add("guide-not-deducible-constant", "template <T, int N> Box(T)", "reject:CTAD-13", lambda w=w: check_user_guide(G([FP(T)], S("Box", T), [tp("T"), tp("N", "const", ctype=I)]), w) or Result(None, ""))
add("guide-not-deducible", "template <T, U> Box(T) -> Box<U>", "reject:CTAD-13", lambda w=w: check_user_guide(G([FP(T)], S("Box", U), [tp("T"), tp("U")]), w) or Result(None, ""))
add("guide-not-deducible-used", "template <T, U> Box(T) -> Box<U>", "reject:CTAD-13", lambda w=w: check_user_guide(G([FP(T)], S("Box", U), [tp("T"), tp("U")]), w) or Result(None, ""))
w = World()
w.templates["Box"] = Box1()
w.guides += [G([FP(Ptr(T))], S("Box", L), [tp("T")]), G([FP(Ptr(T))], S("Box", SH), [tp("T")])]
add("guide-template-same-parameters", "declarations", "accept", lambda w=w: check_user_guide(w.guides[1], w, w.guides) or Result(None, ""))
add("guide-template-same-parameters", "Box b(&i)", "reject:CTAD-24", run1("Box", "paren", [pr(Ptr(I))], w))

w = World()
w.templates["Box"] = ct("Box", [tp("T")], ctors=[Ctor([FP(U)], [tp("U")]), Ctor([FP(T)])])
w.guides.append(UserGuide("Box", [FP(U)], S("Box", CH), [tp("U")], pos=10))
add("guide-after-use", "Box a(1L) before", "Box<long>", run1("Box", "paren", [pr(L)], w, use_pos=5))
add("guide-after-use", "Box b(1L) after", "Box<char>", run1("Box", "paren", [pr(L)], w, use_pos=15))

w = World()
w.templates["Box"] = Box1()
w.templates["Cell"] = ct("Cell", [tp("T")], ctors=[Ctor([FP(T)])])
w.guides += [UserGuide("Box", [FP(I)], S("Box", L), pos=10), UserGuide("Cell", [FP(I)], S("Cell", L), pos=11)]
add("guide-before-instantiation", "make(1)", "Box<long>", run1("Box", "paren", [lv(I)], w, use_pos=20), "caller gives the point")
add("guide-before-instantiation", "fixed<char>()", "Box<long>", run1("Box", "paren", [pr(I)], w, use_pos=20), "caller gives the point")
add("guide-before-instantiation", "early = later(1)", "Cell<int>", run1("Cell", "paren", [lv(I)], w, use_pos=5), "caller gives the point")
add("guide-before-instantiation", "z = later(1)", "Cell<int>", run1("Cell", "paren", [lv(I)], w, use_pos=5), "caller gives the point")

Wide = lambda n: (lambda b: size(b[n]) > 1)
w = World()
w.templates["Box"] = ct("Box", [tp("T")], ctors=[Ctor([FP(U), FP(T)], [tp("U")]), Ctor([FP(T), FP(U)], [tp("U")])])
add("guide-constraints-absent-ambiguous", "Box box(1, 'a')", "reject:CTAD-24", run1("Box", "paren", [pr(I), pr(CH)], w))
w = World()
w.templates["Box"] = ct("Box", [tp("T")], ctors=[Ctor([FP(U), FP(T)], [tp("U")], constraint=Wide("U")),
                                                 Ctor([FP(T), FP(U)], [tp("U")], constraint=Wide("U"))])
w.templates["Pad"] = ct("Pad", [tp("T")], ctors=[Ctor([FP(T)])], constraint=Wide("T"))
add("guide-constraints", "Box box(1, 'a')", "Box<char>", run1("Box", "paren", [pr(I), pr(CH)], w))
add("guide-constraints", "Pad pad(1)", "Pad<int>", run1("Pad", "paren", [pr(I)], w))
add("guide-constraints-none-satisfied", "Box box('a', 'a')", "reject:CTAD-24", run1("Box", "paren", [pr(CH), pr(CH)], w))

# ------------------------------------------------------------------ inherited
def base_world():
    w = World()
    w.templates["Base"] = ct("Base", [tp("T")], ctors=[Ctor([FP(T)])])
    return w


w = base_world()
w.templates["Derived"] = ct("Derived", [tp("T")], inherits=[S("Base", T)])
add("inherited-constructors", "Derived d(1)", "Derived<int>", run1("Derived", "paren", [pr(I)], w))
add("inherited-copy-candidate", "Derived d(b)", "reject:CTAD-26", run1("Derived", "paren", [lv(Cls("Base", (I,), cvs(C), True))], w))
w.guides.append(UserGuide("Derived", [FP(I), FP(I)], S("Derived", CH)))
add("inherited-copy-candidate-user-guide", "Derived d(b)", "reject:CTAD-24", run1("Derived", "paren", [lv(Cls("Base", (I,), cvs(C), True))], w))
w = World()
w.templates["B"] = ct("B", [tp("T")], ctors=[Ctor([FP(T)])])
w.templates["D"] = ct("D", [tp("T"), tp("U", default=I)], inherits=[S("B", T)])
add("inherited-guide-unused-parameter-default", "D d(1.5)", "D<double, int>", run1("D", "paren", [pr(DB)], w))
w = base_world()
w.templates["Derived"] = ct("Derived", [tp("T")], inherits=[S("Base", I)])
add("inherited-not-deducible", "Derived d(1)", "reject:CTAD-24", run1("Derived", "paren", [pr(I)], w))
w = World()
w.templates["Base"] = ct("Base", [tp("T")], ctors=[Ctor([FP(U)], [tp("U")])])
w.guides.append(UserGuide("Base", [FP(Ptr(T))], S("Base", T), [tp("T")]))
w.templates["Derived"] = ct("Derived", [tp("T")], inherits=[S("Base", T)])
add("inherited-user-guide", "Derived d(&n)", "Derived<int>", run1("Derived", "paren", [pr(Ptr(I))], w))
w = base_world()
w.templates["Derived"] = ct("Derived", [tp("T")], aggregate=True, elements=[Element(S("Base", T), "base")])
add("inherited-without-using", "Derived d(1)", "reject:CTAD-24", run1("Derived", "paren", [pr(I)], w))

# ------------------------------------------------------------------ lists (CTAD-18)
w = World()
w.templates["List"] = ct("List", [tp("T")], ctors=[Ctor([FP(IL(T))])])
w.aliases["Alias"] = AliasTemplate("Alias", [tp("T")], S("List", T))
w.env.classes["More"] = ClassDef(bases=[S("List", I)])
oneL = lv(Cls("List", (I,), cvs(C), True))
add("list-single-specialization-alias", "Alias copy{one}", "List<int>", run1("Alias", "list", [oneL], w))
add("list-single-specialization-alias", "Alias two{one, one}", "List<List<int>>", run1("Alias", "list", [oneL, oneL], w))
add("list-single-specialization", "List one{1, 2}", "List<int>", run1("List", "list", [pr(I), pr(I)], w))
add("list-single-specialization", "List copy{one}", "List<int>", run1("List", "list", [oneL], w))
add("list-single-specialization", "List copy_list = {one}", "List<int>", run1("List", "copy-list", [oneL], w))
add("list-single-specialization", "List nested{one, one}", "List<List<int>>", run1("List", "list", [oneL, oneL], w))
add("list-single-specialization", "List from_derived{more}", "List<int>", run1("List", "list", [lv(Cls("More", (), cvs(C)))], w))
add("list-single-specialization", "List single{1L}", "List<long>", run1("List", "list", [pr(L)], w))

w = World()
w.templates["In"] = ct("In", [tp("T")], ctors=[Ctor([FP(U)], [tp("U")])])
w.guides.append(UserGuide("In", [FP(Ptr(T))], S("In", T), [tp("T")]))
add("member-guide", "Outer::In i(&n)", "In<int>", run1("In", "paren", [pr(Ptr(I))], w))

w = World()
w.templates["Tag"] = ct("Tag", [tp("T", default=I)])
add("no-constructor", "Tag a", "Tag<int>", run1("Tag", "none", [], w))
add("no-constructor", "Tag b{}", "Tag<int>", run1("Tag", "list", [], w))
w = World()
w.templates["Box"] = Box1()
add("no-initializer", "Box b", "reject:CTAD-24", run1("Box", "none", [], w))
add("partial-argument-list", "Pair<int> p(1, 2.5): no deduction", "accept", lambda: check_place("variable", has_list=True) or Result(None, ""))

# ------------------------------------------------------------------ initialization
w = World()
w.templates["Pair"] = ct("Pair", [tp("T")], ctors=[Ctor([FP(T), FP(L)]), Ctor([FP(L), FP(T)])])
w.guides.append(UserGuide("Pair", [FP(T), FP(T)], S("Pair", T), [tp("T")]))
add("result-then-initialize", "Pair p(1, 2)", "reject:CTAD-26", run1("Pair", "paren", [pr(I), pr(I)], w))
w = World()
w.templates["Gone"] = ct("Gone", [tp("T")], ctors=[Ctor([FP(T)], deleted=True)])
add("result-then-initialize-deleted", "Gone g(1)", "reject:CTAD-26", run1("Gone", "paren", [pr(I)], w))
w = World()
w.templates["Wide"] = ct("Wide", [tp("T")], ctors=[Ctor([FP(T)])])
w.guides.append(UserGuide("Wide", [FP(T)], S("Wide", LL), [tp("T")]))
add("result-type", "Wide w(2)", "Wide<long long>", run1("Wide", "paren", [pr(I)], w))

w = World()
w.templates["Pick"] = ct("Pick", [tp("T")], ctors=[Ctor([FP(Ptr(T))], explicit=True), Ctor([FP(T)])])
add("selection-forms", "Pick a(&n)", "Pick<int>", run1("Pick", "paren", [pr(Ptr(I))], w))
add("selection-forms", "Pick b = &n", "Pick<int*>", run1("Pick", "copy", [pr(Ptr(I))], w))
w = World()
w.templates["Secret"] = ct("Secret", [tp("T")], ctors=[Ctor([FP(Ptr(T))], access="private"), Ctor([FP(U)], [tp("U")])])
add("selection-private-constructor", "Secret s(&n)", "reject:CTAD-26", run1("Secret", "paren", [pr(Ptr(I))], w))

w = World()
w.templates["Box"] = Box1()
add("several-declarators-copy-init", "Box e = 1, f = 'c'", "reject:CTAD-3",
    lambda w=w: ctad_declarators("Box", [Init("copy", [pr(I)]), Init("copy", [pr(CH)])], w))
add("several-declarators", "Box a(1), b(2)", "Box<int>", lambda w=w: ctad_declarators("Box", [Init("paren", [pr(I)]), Init("paren", [pr(I)])], w))
add("several-declarators", "Box c(1), d(2.0)", "reject:CTAD-3", lambda w=w: ctad_declarators("Box", [Init("paren", [pr(I)]), Init("paren", [pr(DB)])], w))

for t, place in [("site-array", "array"), ("site-member", "nonstatic-member"), ("site-parameter", "function-param"),
                 ("site-pointer", "pointer"), ("site-reference", "reference"), ("site-return-type", "return")]:
    add(t, place, "reject:CTAD-2", (lambda p: lambda: check_place(p) or Result(None, ""))(place))

w = World()
w.templates["Box"] = Box1()
for lab, form, arg, want in [("Box a(1)", "paren", pr(I), "Box<int>"), ("Box b{2L}", "list", pr(L), "Box<long>"),
                             ("Box c = 'c'", "copy", pr(CH), "Box<char>"), ("Box d = {1.0}", "copy-list", pr(DB), "Box<double>"),
                             ("const Box e(1u)", "paren", pr(UN), "Box<unsigned int>"), ("new Box(3)", "paren", pr(I), "Box<int>"),
                             ("new const Box{4}", "list", pr(I), "Box<int>"), ("Box(1)", "paren", pr(I), "Box<int>"),
                             ("Box{1u}", "list", pr(UN), "Box<unsigned int>"),
                             ("static constexpr Box member = Box<int>(1)", "copy", pr(S("Box", I)), "Box<int>"),
                             ("static inline Box other = 2", "copy", pr(I), "Box<int>")]:
    add("sites", lab, want, run1("Box", form, [arg], w))
for place in ["variable", "static-member-init", "new-init", "functional-cast", "constant-param"]:
    add("sites", "place: " + place, "accept", (lambda p: lambda: check_place(p) or Result(None, ""))(place))

w = World()
w.templates["Pair"] = ct("Pair", [tp("A"), tp("B")], aggregate=True, elements=[Element(A_, name="a"), Element(B_, name="b")])
w.templates["Box"] = Box1()
add("sites-for-and-condition", "for (Pair p : items)", "Pair<int, char>", run1("Pair", "copy", [lv(S("Pair", I, CH))], w))
add("sites-for-and-condition", "if (Box b = Box(2))", "Box<int>", run1("Box", "copy", [pr(S("Box", I))], w))
add("sites-for-and-condition", "if (Box b{3})", "Box<int>", run1("Box", "list", [pr(I)], w))
for place in ["range-for", "condition"]:
    add("sites-for-and-condition", "place: " + place, "accept", (lambda p: lambda: check_place(p) or Result(None, ""))(place))

w = World()
w.templates["Shape"] = ct("Shape", [tp("T")], ctors=[Ctor([FP(T)])])
add("specialization-constructors", "Shape s(&i, 1)", "reject:CTAD-24", run1("Shape", "paren", [pr(Ptr(I)), pr(I)], w))

w = World()
w.templates["Box"] = Box1()
w.aliases["A"] = AliasTemplate("A", [tp("T")], S("Box", T))
add("template-template-parameter-alias", "A a(1), W = Box", "Box<int>", run1("A", "paren", [pr(I)], w), "after substitution")
add("template-template-parameter", "W w(1), W = Box", "Box<int>", run1("Box", "paren", [pr(I)], w), "after substitution")

# ------------------------------------------------------------------ ties
w = World()
w.templates["Copy"] = ct("Copy", [tp("T")], ctors=[Ctor([FP(T)]), Ctor([FP(LRef(Cls("Copy", (T,), cvs(C), True)))])])
add("tie-copy-candidate", "Copy two(one)", "Copy<int>", run1("Copy", "paren", [lv(Cls("Copy", (I,), cvs(C), True))], w))
add("tie-copy-candidate-forms", "Copy two = one", "Copy<char>", run1("Copy", "copy", [lv(Cls("Copy", (CH,), cvs(C), True))], w))
add("tie-copy-candidate-forms", "Copy three{one}", "Copy<char>", run1("Copy", "list", [lv(Cls("Copy", (CH,), cvs(C), True))], w))

w = World()
w.templates["Mark"] = ct("Mark", [tp("T")], ctors=[Ctor([FP(T)]), Ctor([FP(T), FP(Ptr(U), default=True)], [tp("U", default=I)], explicit=True)])
add("tie-non-template", "Mark m = {1}", "Mark<int>", run1("Mark", "copy-list", [pr(I)], w))
w = World()
w.templates["Mark"] = ct("Mark", [tp("T")], ctors=[Ctor([FP(T)], explicit=True), Ctor([FP(T), FP(Ptr(U), default=True)], [tp("U", default=I)])])
add("tie-non-template-explicit", "Mark m = {1}", "reject:CTAD-23", run1("Mark", "copy-list", [pr(I)], w))

w = World()
w.templates["Base"] = ct("Base", [tp("T")], ctors=[Ctor([FP(T)], explicit=True)])
w.templates["Derived"] = ct("Derived", [tp("T")], ctors=[Ctor([FP(T)])], inherits=[S("Base", T)])
add("tie-own-constructor", "Derived d = {1}", "Derived<int>", run1("Derived", "copy-list", [pr(I)], w))
w = base_world()
w.templates["Derived"] = ct("Derived", [tp("T")], ctors=[Ctor([FP(T), FP(Ptr(U), default=True)], [tp("U", default=I)], explicit=True)], inherits=[S("Base", T)])
add("tie-own-constructor-template", "Derived d = {1}", "reject:CTAD-23", run1("Derived", "copy-list", [pr(I)], w))
w = base_world()
w.templates["Derived"] = ct("Derived", [tp("T")], ctors=[Ctor([FP(LRef(D.add_cv(T, cvs(C))))])], inherits=[S("Base", T)])
add("tie-own-constructor-types-differ", "Derived d(1)", "reject:CTAD-24", run1("Derived", "paren", [pr(I)], w))
w = World()
w.templates["Base"] = ct("Base", [tp("T")], ctors=[Ctor([FP(U)], [tp("U")])])
w.guides.append(UserGuide("Base", [FP(T)], S("Base", T), [tp("T")]))
w.templates["Derived"] = ct("Derived", [tp("T")], ctors=[Ctor([FP(T)], explicit=True)], inherits=[S("Base", T)])
add("tie-own-constructor-user-guide", "Derived d = {1}", "reject:CTAD-23", run1("Derived", "copy-list", [pr(I)], w))

w = World()
w.templates["Box"] = Box1()
w.guides.append(UserGuide("Box", [FP(I)], S("Box", L)))
add("tie-user-guide-conversion", "Box a(small)", "Box<short>", run1("Box", "paren", [lv(Prim("short", cvs(C)))], w))
add("tie-user-guide-conversion", "Box b(1)", "Box<long>", run1("Box", "paren", [pr(I)], w))
w = World()
w.templates["Box"] = Box1()
w.guides.append(UserGuide("Box", [FP(S("Box", T))], S("Box", S("Box", T)), [tp("T")]))
add("tie-user-guide-copy", "Box two(one)", "Box<Box<int>>", run1("Box", "paren", [lv(Cls("Box", (I,), cvs(C), True))], w))
w = World()
w.templates["Box"] = Box1()
w.guides.append(UserGuide("Box", [FP(T)], S("Box", L), [tp("T")]))
add("tie-user-guide", "Box b(1)", "Box<long>", run1("Box", "paren", [pr(I)], w))

Ts, Us = TParam("Ts"), TParam("Us")
w = World()
w.templates["Pack"] = ct("Pack", [tp("Ts", pack=True)], ctors=[Ctor([FP(Us, pack=True)], [tp("Us", pack=True)],
                         constraint=lambda b: len(b["Us"]) == len(b["Ts"]) and len(b["Ts"]) > 0)])
w.guides.append(UserGuide("Pack", [FP(Us, pack=True)], S("Pack", Expand(Us)), [tp("Us", pack=True)]))
add("user-guide-beside-constrained-constructor-template", "Pack(1, 'x')", "Pack<int, char>", run1("Pack", "paren", [pr(I), pr(CH)], w))

w = World()
w.templates["Box"] = ct("Box", [tp("T")], ctors=[Ctor([FP(U)], [tp("U")]), Ctor([FP(U), FP(V_)], [tp("U"), tp("V")])])
w.guides.append(UserGuide("Box", [FP(RRef(T))], S("Box", T), [tp("T")], pos=1))
add("user-guides", "Box a(n)", "Box<int&>", run1("Box", "paren", [lv(I)], w, use_pos=2))
add("user-guides", "Box b(1)", "Box<int>", run1("Box", "paren", [pr(I)], w, use_pos=2))
w.guides.append(UserGuide("Box", [FP(Ptr(CCH))], S("Box", L), pos=3))
add("user-guides", "Box c(\"text\")", "Box<long>", run1("Box", "paren", [lit(5)], w, use_pos=4))
w.guides.append(UserGuide("Box", [FP(Ptr(T)), FP(Ptr(T))], S("Box", U), [tp("T"), tp("U", default=SH)], pos=5))
add("user-guides", "Box d(&n, &n)", "Box<short>", run1("Box", "paren", [pr(Ptr(I)), pr(Ptr(I))], w, use_pos=6))
w.templates["Item"] = ct("Item", [tp("T")], ctors=[Ctor([FP(U, pack=True)], [tp("U", pack=True)])])
w.guides.append(UserGuide("Item", [FP(Ptr(T)), FP(I, default=True)], S("Item", T), [tp("T")], pos=7))
add("user-guides", "Item e(&n)", "Item<int>", run1("Item", "paren", [pr(Ptr(I))], w, use_pos=8))


# ------------------------------------------------------------------ tests outside the page's directory
Fs = TParam("Fs")
w = World()
w.templates["Overload"] = ct("Overload", [tp("Fs", pack=True)], aggregate=True, elements=[Element(Expand(Fs), "base")])
add("expressions/lambdas/closure-as-base-overload-set", "Overload o{[](int){}, [](auto){}}", "Overload<c1, c2>",
    run1("Overload", "list", [pr(Cls("c1")), pr(Cls("c2"))], w))
add("programs/W18/visitor", "Overload{lambdas...}", "Overload<c1, c2>", run1("Overload", "list", [pr(Cls("c1")), pr(Cls("c2"))], w))

w = World()
w.templates["Wrap"] = ct("Wrap", [tp("T")], ctors=[Ctor([FP(T)])])
add("templates/declarations/constant-parameter-class-template", "Wrap{3}", "Wrap<int>", run1("Wrap", "list", [pr(I)], w))
add("templates/declarations/constant-parameter-class-template", "Hold<4>", "Wrap<int>", run1("Wrap", "copy", [pr(I)], w))
add("templates/declarations/constant-parameter-class-template", "Hold<'a'>", "Wrap<char>", run1("Wrap", "copy", [pr(CH)], w))

w = World()
w.templates["Unexpected"] = ct("Unexpected", [tp("E")], aggregate=True, elements=[Element(TParam("E"), name="error")])
for prog in ("roadmap/r11-expected", "roadmap/r11-expected-first-try"):
    add("programs/" + prog, "Unexpected{Err::not_digit}", "Unexpected<Err>", run1("Unexpected", "list", [pr(Cls("Err"))], w))
w = World()
w.templates["Err"] = ct("Err", [tp("E")], aggregate=True, elements=[Element(TParam("E"), name="error")])
add("programs/P1/parse", "Err{ParseError::Empty()}", "Err<ParseError>", run1("Err", "list", [pr(Cls("ParseError"))], w))

w = World()
w.templates["Defer"] = ct("Defer", [tp("F")], ctors=[Ctor([FP(TParam("F"))], explicit=True),
                                                    Ctor([FP(LRef(Cls("Defer", (TParam("F"),), cvs(C), True)))], deleted=True)])
add("programs/W10/cleanup_guard", "Defer a([&]{...})", "Defer<closure>", run1("Defer", "paren", [pr(Cls("closure"))], w))

M = CParam("M")
SZ = Prim("unsigned long")
w = World()
w.templates["SmallString"] = ct("SmallString", [tp("N", "const", ctype=SZ)], ctors=[
    Ctor([]), Ctor([FP(LRef(Arr(CCH, M)))], [tp("M", "const", ctype=SZ)],
                   constraint=lambda b: "N" in b and b["M"].value - 1 <= b["N"].value),
    Ctor([FP(IL(CH))])])
mfn = lambda b: Val(b["M"].value - 1, SZ)
mfn.names = ("M",)
w.guides.append(UserGuide("SmallString", [FP(LRef(Arr(CCH, M)))], S("SmallString", D.ValExpr(mfn, "M - 1")),
                          [tp("M", "const", ctype=SZ)]))
add("programs/W7/small_string", "SmallString hello = \"hello\"", "SmallString<5>", run1("SmallString", "copy", [lit(6)], w))
add("programs/W7/small_string", "SmallString(\" you\")", "SmallString<4>", run1("SmallString", "paren", [lit(5)], w))

w = World()
w.templates["Pair"] = ct("Pair", [tp("A"), tp("B")], aggregate=True, elements=[Element(A_, name="first"), Element(B_, name="second")])
w.guides.append(UserGuide("Pair", [FP(A_), FP(B_)], S("Pair", A_, B_), [tp("A"), tp("B")]))
add("programs/roadmap/r6-text", "Pair p{1, 2.5}", "Pair<int, double>", run1("Pair", "list", [pr(I), pr(DB)], w))

w = World()
w.templates["Slice"] = ct("Slice", [tp("T")], ctors=[Ctor([]), Ctor([FP(LRef(Arr(D.add_cv(T, cvs(C)), N)))], [tp("N", "const", ctype=SZ)])])
add("programs/P1/stats", "Slice(words)", "Slice<Word>", run1("Slice", "paren", [lv(Arr(Cls("Word"), 3))], w))

R_, P_ = TParam("R"), TParam("Pred")
w = World()
w.templates["filter_view"] = ct("filter_view", [tp("R"), tp("Pred")], ctors=[Ctor([FP(LRef(D.add_cv(R_, cvs(C)))), FP(P_)])])
add("programs/roadmap/r8-pipeline", "filter_view(r, a.f)", "filter_view<FixedVec<int>, closure>",
    run1("filter_view", "paren", [lv(Cls("FixedVec", (I,), cvs(C), True)), lv(Cls("closure"))], w), "after substitution")

w = World()
w.templates["Span"] = ct("Span", [tp("T")], ctors=[Ctor([FP(Ptr(T)), FP(SZ)]), Ctor([FP(LRef(Arr(T, N)))], [tp("N", "const", ctype=SZ)])])
w.templates["Indirect"] = ct("Indirect", [tp("Less", default=Cls("LessThan"))], aggregate=True, elements=[Element(TParam("Less"), name="less")])
add("programs/W20/span_sort", "Span(numbers)", "Span<int>", run1("Span", "paren", [lv(Arr(I, 6))], w))
add("programs/W20/span_sort", "Indirect{lambda}", "Indirect<closure>", run1("Indirect", "list", [pr(Cls("closure"))], w))

w = World()
w.templates["tuple_base"] = ct("tuple_base", [tp("Seq"), tp("T", pack=True)], defined=False)
w.templates["Tuple"] = ct("Tuple", [tp("T", pack=True)], inherits=[S("tuple_base", Cls("seq"), Expand(T))])
w.guides.append(UserGuide("Tuple", [FP(T, pack=True)], S("Tuple", Expand(T)), [tp("T", pack=True)]))
add("programs/roadmap/r8-tasks", "Tuple t{1, 'x', 2L}", "Tuple<int, char, long>", run1("Tuple", "list", [pr(I), pr(CH), pr(L)], w))
add("programs/roadmap/r8-tasks", "Tuple u{3, 4, 5}", "Tuple<int, int, int>", run1("Tuple", "list", [pr(I), pr(I), pr(I)], w))

w = World()
w.templates["Span"] = ct("Span", [tp("T")], ctors=[Ctor([FP(LRef(Arr(T, N)))], [tp("N", "const", ctype=SZ)])])
w.guides.append(UserGuide("Span", [FP(LRef(Arr(T, N)))], S("Span", T), [tp("T"), tp("N", "const", ctype=SZ)]))
w.templates["Filter"] = ct("Filter", [tp("R"), tp("P")], ctors=[Ctor([FP(R_), FP(TParam("P"))])])
w.templates["Adaptor"] = ct("Adaptor", [tp("F")], aggregate=True, elements=[Element(TParam("F"), name="make")])
add("programs/roadmap/r11-views", "Span(data)", "Span<int>", run1("Span", "paren", [lv(Arr(I, 5))], w))
add("programs/roadmap/r11-views", "Filter(move(r), pred)", "Filter<Span<int>, closure>",
    run1("Filter", "paren", [Arg(S("Span", I), "xvalue"), lv(Cls("closure"))], w), "after substitution")
add("programs/roadmap/r11-views", "Adaptor([pred](auto r){...})", "Adaptor<closure>", run1("Adaptor", "paren", [pr(Cls("closure"))], w))

NOT_EXPRESSIBLE = {
    "guide-access": "CTAD-12 access: the tree has no access for a guide or a member template",
    "guide-definition": "CTAD-12 body: the tree has no guide body",
    "guide-specifier": "CTAD-12 specifier: the tree has no specifiers other than explicit",
    "guide-wrong-scope": "CTAD-12 scope: the tree has no scopes",
}

# ------------------------------------------------------------------ run
def show(r):
    if r is None:
        return "accept"
    if r.kind == "fail":
        return f"reject:{r.rule}"
    if r.type is None:
        return "accept"
    return repr(r.type).replace("'", "")


def norm(s):
    return s.replace("const ", "").replace(" ", "")


right = wrong = rule_ok = rule_n = 0
by_test = {}
bad = []
for test, label, exp, thunk, note in A:
    try:
        r = thunk()
        got = show(r)
    except Exception as e:  # a crash is a wrong answer
        got = f"crash: {type(e).__name__}: {e}"
    if exp.startswith("reject"):
        ok = got.startswith("reject")
        if ok and ":" in exp:
            rule_n += 1
            rule_ok += got == exp
    elif exp == "accept":
        ok = not got.startswith("reject") and not got.startswith("crash")
    else:
        ok = norm(got) == norm(exp)
    right += ok
    wrong += not ok
    by_test.setdefault(test, []).append(ok)
    if not ok:
        bad.append((test, label, exp, got))
    print(("ok   " if ok else "WRONG ") + f"{test}: {label}: want {exp}, got {got}" + (f"  [{note}]" if note else ""))

print()
print(f"assertions: {right} of {right + wrong} right")
print(f"tests: {sum(all(v) for v in by_test.values())} of {len(by_test)} with every assertion right")
print(f"rule of a reject: {rule_ok} of {rule_n} name the test's rule")
print(f"not expressible: {len(NOT_EXPRESSIBLE)} tests: " + ", ".join(NOT_EXPRESSIBLE))
print("wrong:")
for b in bad:
    print("  ", *b, sep=" | ")
