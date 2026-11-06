#!/usr/bin/env python3
"""Score the frozen deduce.py against the tests that cite DED rules.
Each case is one assertion of a test. Expected values come from the test's
static_assert, its tidal-expect line, or the grid tables."""
import hashlib, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
FROZEN = open(os.path.join(HERE, "deduce.sha1")).read().split()[0]
assert hashlib.sha1(open(os.path.join(HERE, "deduce.py"), "rb").read()).hexdigest() == FROZEN, "deduce.py changed"
sys.path.insert(0, HERE)
from deduce import *  # noqa

# ---------------------------------------------------------------- shorthands
T, U, R, Ts, A_ = TParam("T"), TParam("U"), TParam("R"), TParam("Ts"), TParam("A")
I, L, CH, D, F_, SH = INT, Prim("long"), Prim("char"), Prim("double"), Prim("float"), Prim("short")
cI = Prim("int", cvs(C))


def c(t):
    return add_cv(t, {C})


def lv(t):
    return Arg(t, "lvalue")


def xv(t):
    return Arg(t, "xvalue")


def pv(t):
    return Arg(t, "prvalue")


def tmpl(tps, params, ret=VOID, **kw):
    out = []
    for p in tps:
        if isinstance(p, TP):
            out.append(p)
        else:
            out.append(TP(p))
    return Template("f", out, [p if isinstance(p, FP) else FP(p) for p in params], ret, **kw)


def spec(name, *args):
    return Cls(name, tuple(args), is_template=True)


def plain(params, variadic=False, ret=VOID, defaults=0):
    return Plain("f", Func(ret, tuple(params), variadic), defaults)


ELL = plain([], True)
results = []


def record(test, what, ok, why=""):
    results.append((test, what, ok, why))


def got_args(r):
    return r.args if isinstance(r, Deduced) and r.viable else None


def eq(r, want):
    a = got_args(r)
    if a is None:
        return False
    return all(a.get(k) == v for k, v in want.items())


def chk(test, what, t, args, want, explicit=(), env=None):
    r = deduce(t, args, explicit, env or Env())
    ok = eq(r, want)
    record(test, what, ok, "" if ok else f"got {r}")


def chk_ret(test, what, t, args, want, explicit=(), env=None):
    r = deduce(t, args, explicit, env or Env())
    ok = isinstance(r, Deduced) and r.viable and r.ret == want
    record(test, what, ok, "" if ok else f"got {r}")


def not_eq(test, what, t, args, want, explicit=(), env=None):
    """The static_assert is false: deduction fails or gives other values (grid cells)."""
    r = deduce(t, args, explicit, env or Env())
    ok = not eq(r, want)
    record(test, what, ok, "" if ok else f"got {r}")


def rejects(test, what, cands, args, explicit=(), env=None, rule=None):
    r = resolve(cands, args, explicit, env or Env())
    ok = isinstance(r, Reject)
    why = "" if ok else f"got {r}"
    if ok and rule and r.rule != rule:
        why = f"rule {r.rule}, test names {rule}"
    record(test, what, ok, why)
    inner = [deduce(t, args, explicit, env or Env()) for t in cands if isinstance(t, Template)]
    inner_rules.append((test, what, [getattr(d, "rule", "viable=False" if isinstance(d, Deduced) and not d.viable else "deduced") for d in inner]))


inner_rules = []


def picks(test, what, cands, args, index, explicit=(), env=None, want=None):
    r = resolve(cands, args, explicit, env or Env())
    ok = isinstance(r, Chosen) and r.index == index
    if ok and want:
        ok = eq(r.result, want)
    record(test, what, ok, "" if ok else f"got {r}")


skipped = []


def skip(test, n, why):
    skipped.append((test, n, why))


# ---------------------------------------------------------------- environments
def env_basic():
    e = Env()
    e.class_templates["Box"] = lambda a: ClassDef()
    e.ctemplate_params["Box"] = [TP("T")]
    return e


Circle = Cls("Circle")

# ---------------------------------------------------------------- address-*
e = Env(); e.classes["Circle"] = ClassDef()
g_t = tmpl(["T"], [LRef(c(T))], D)
h_t = tmpl(["T"], [LRef(c(T))], Decltype(lambda b, env: (_ for _ in ()).throw(SubstFail("DED-24", "no side")), "decltype(t.side)"), trailing=True)
target = Func(I, (LRef(c(Circle)),))
r1, r2 = deduce_target(g_t, target, e), deduce_target(h_t, target, e)
record("address-declared-result-body-not-instantiated", "g: template does not match int(const Circle&)", isinstance(r1, Failure), str(r1))
record("address-declared-result-body-not-instantiated", "h: template removed or loses", not isinstance(r2, HardError), str(r2))
g_auto = tmpl(["T"], [LRef(c(T))], VOID, deduced_ret=True, body_error=True)
for name in ("address-deduced-result-body-instantiated", "address-deduced-result-body-static-cast"):
    r = deduce_target(g_auto, target, e)
    record(name, "body of the deduced result type is instantiated: reject (DED-29)", isinstance(r, HardError),
           f"got {r}: deduce_target never instantiates a deduced result type")

# ---------------------------------------------------------------- adjust-*
n, cc, arr = lv(I), lv(cI), lv(Arr(I, 3))
ref, cref, cval, val = tmpl(["T"], [LRef(T)]), tmpl(["T"], [LRef(c(T))]), tmpl(["T"], [c(T)]), tmpl(["T"], [T])
tn = "adjust-reference-parameter"
chk(tn, "ref(c)", ref, [cc], {"T": cI})
chk(tn, "ref(arr)", ref, [arr], {"T": Arr(I, 3)})
chk(tn, "cref(c)", cref, [cc], {"T": I})
chk(tn, "cref(1)", cref, [pv(I)], {"T": I})
chk(tn, "cref(arr)", cref, [arr], {"T": Arr(I, 3)})
chk(tn, "cval(c)", cval, [cc], {"T": I})
chk(tn, "val(xvalue)", val, [xv(I)], {"T": I})
rejects("adjust-reference-rvalue", "ref(1)", [ref], [pv(I)], rule="DED-35")
tn = "adjust-value-parameter"
fnc = Func(I, (CH,))
chk(tn, "take(arr)", val, [arr], {"T": Ptr(I)})
chk(tn, "take(fn)", val, [lv(fnc)], {"T": Ptr(fnc)})
chk(tn, "take(c)", val, [cc], {"T": I})
chk(tn, "take(\"ab\")", val, [lv(Arr(c(CH), 3))], {"T": Ptr(c(CH))})
chk(tn, "take(cp)", val, [lv(Ptr(cI, cvs(C)))], {"T": Ptr(cI)})
tn = "adjust-value-parameter-more"
chk(tn, "take(volatile int)", val, [lv(Prim("int", cvs(V)))], {"T": I})
chk(tn, "take(grid)", val, [lv(Arr(Arr(I, 3), 2))], {"T": Ptr(Arr(I, 3))})
chk(tn, "take(const int[2])", val, [lv(Arr(cI, 2))], {"T": Ptr(cI)})

# array-bound-from-converted-elements
e = Env(); e.classes["Name"] = ClassDef(converts_from=[Ptr(c(CH))])
ext = tmpl(["T", TP("N", "const", ctype=SIZE_T)], [LRef(Arr(T, CParam("N")))], SIZE_T)
chk("array-bound-from-converted-elements", "extent_of(names) N = 4", ext, [lv(Arr(Cls("Name"), 4))], {"N": Val(4, SIZE_T)}, env=e)

# ---------------------------------------------------------------- braced-*
lenT = tmpl(["T", TP("N", "const", ctype=I)], [LRef(Arr(c(T), CParam("N")))], I)
len2 = plain([LRef(Arr(c(L), 2))], ret=I)
tn = "braced-array-reference-conflict"
picks(tn, "len({1.5, 2.5}) == 2", [lenT, len2], [Braced([pv(D), pv(D)])], 0, want={"N": Val(2, I)})
picks(tn, "len({1, 2L}) == 20", [lenT, len2], [Braced([pv(I), pv(L)])], 1)
area = tmpl(["T", TP("M", "const", ctype=I), TP("N", "const", ctype=I)], [LRef(Arr(Arr(c(T), CParam("N")), CParam("M")))], I)
chk(tn, "area({{1,2,3}}) == 13", area, [Braced([Braced([pv(I), pv(I), pv(I)])])], {"M": Val(1, I), "N": Val(3, I)})
tn = "braced-array-reference"
chk(tn, "len({1,2,3}) == 3", lenT, [Braced([pv(I)] * 3)], {"N": Val(3, I)})
elem = tmpl(["T", TP("N", "const", ctype=I)], [RRef(Arr(T, CParam("N")))])
chk(tn, "elem({'a','b'}) T = char", elem, [Braced([pv(CH), pv(CH)])], {"T": CH})
chk(tn, "area 3x2", area, [Braced([Braced([pv(I), pv(I)])] * 3)], {"M": Val(3, I), "N": Val(2, I)})
IL = lambda t: spec("std::initializer_list", t)
lst = tmpl(["T"], [IL(T)])
lst_ref = tmpl(["T"], [LRef(c(IL(T)))])
rejects("braced-elements-differ", "list({1, 2L})", [lst], [Braced([pv(I), pv(L)])], rule="DED-35")
rejects("braced-empty", "list({})", [lst], [Braced([])], rule="DED-35")
tn = "braced-initializer-list"
chk(tn, "list({1,2,3})", lst, [Braced([pv(I)] * 3)], {"T": I})
chk(tn, "list({'a'})", lst, [Braced([pv(CH)])], {"T": CH})
chk(tn, "list_ref({1.0, 2.0})", lst_ref, [Braced([pv(D), pv(D)])], {"T": D})
rejects("braced-other-parameter", "one({1})", [val], [Braced([pv(I)])], rule="DED-35")
e = env_basic()
put = tmpl(["T"], [T, spec("Box", T)])
chk("braced-other-parameter-fixed", "put(1, {2})", put, [pv(I), Braced([pv(I)])], {"T": I}, env=e)
Fp = Ptr(Func(VOID, (I,)))
chk("braced-overload-set-element", "list({f, act})", lst,
    [Braced([lv(Fp), OverloadArg([Func(VOID, (I,)), Func(VOID, (L,))])])], {"T": Fp})

# ---------------------------------------------------------------- call-*
pair = tmpl(["T", "U"], [T, U])
chk("call-each-argument", "pair(1,'c')", pair, [pv(I), pv(CH)], {"T": I, "U": CH})
chk("call-each-argument", "pair(1.0, 2L)", pair, [pv(D), pv(L)], {"T": D, "U": L})
chk("call-each-argument", "constructor template Box('c')", val, [pv(CH)], {"T": CH})
tn = "call-member-and-operator"
chk(tn, "Box{}.get(2.0)", val, [pv(D)], {"T": D})
chk(tn, "Box::point(&n)", tmpl(["T"], [Ptr(T)]), [pv(Ptr(I))], {"T": I})
e = Env(); e.classes["Box"] = ClassDef()
chk(tn, "Box{} + 'c'", tmpl(["T"], [Cls("Box"), T]), [pv(Cls("Box")), pv(CH)], {"T": CH}, env=e)

# ---------------------------------------------------------------- combine-*
keep = Func(I, (I,))
chk("combine-agree", "apply(keep, 1)", tmpl(["T"], [Ptr(Func(T, (T,))), T]), [lv(keep), pv(I)], {"T": I})
sized = tmpl(["T", TP("N", "const", ctype=T)], [LRef(Arr(I, CParam("N")))])
chk("combine-bound-alone", "sized(arr) T = size type", sized, [lv(Arr(I, 3))], {"T": SIZE_T})
chk("combine-bound-alone", "bound(arr) == 3", sized, [lv(Arr(I, 3))], {"N": Val(3, SIZE_T)})
sized2 = tmpl(["T", TP("N", "const", ctype=T)], [LRef(Arr(I, CParam("N"))), T])
chk("combine-bound-yields", "sized(arr, short) T = short", sized2, [lv(Arr(I, 3)), pv(SH)], {"T": SH})
larger = tmpl(["T"], [T, T], T)
rejects("combine-conflict", "larger(1, 2L)", [larger], [pv(I), pv(L)], rule="DED-35")
shrink = Func(CH, (I,))
rejects("combine-conflict-one-pair", "apply(shrink)", [tmpl(["T"], [Ptr(Func(T, (T,)))])], [lv(shrink)], rule="DED-35")
e = env_basic()
wrap = tmpl(["T", TP("U", default=spec("Box", T))], [T])
chk("combine-defaults", "wrap(1) U = Box<int>", wrap, [pv(I)], {"U": spec("Box", I)}, env=e)
chk("combine-defaults", "rest(1) Rest empty", tmpl(["T", TP("Rest", pack=True)], [T]), [pv(I)], {"Rest": ()})
chk("combine-leading-pack-empty", "lead(1) P empty", tmpl([TP("P", pack=True), "T"], [T]), [pv(I)], {"P": (), "T": I})
rejects("combine-missing", "make(1)", [tmpl(["T", "U"], [T], U)], [pv(I)], rule="DED-35")

# ---------------------------------------------------------------- constraint-*
e = Env()
e.class_templates["Box"] = lambda a: ClassDef()
e.ctemplate_constraints["Box"] = lambda a: not (a and a[0] in (CH, BOOL, Prim("signed char"), Prim("unsigned char")))
wide = lambda b: b["T"] not in (CH,)
pk = tmpl(["T", TP("B", default=spec("Box", T))], [T], I, constraint=wide)
picks("constraint-failure-in-default-template-argument", "pick('a') == 2", [pk, plain([], True, I)], [pv(CH)], 1, env=e)
picks("constraint-failure-in-default-template-argument", "pick(5) == 1", [pk, plain([], True, I)], [pv(I)], 0, env=e)
mk = tmpl(["T"], [T], spec("Box", T))
picks("constraint-failure-in-return-type", "make('a') == 7", [mk, plain([], True, I)], [pv(CH)], 1, env=e)
picks("constraint-failure-in-return-type", "make(5).v == 5", [mk, plain([], True, I)], [pv(I)], 0, env=e)

# ---------------------------------------------------------------- context-address*
tn = "context-address"
Tag = lambda *a: spec("Tag", *a)
take = tmpl(["T"], [T], Tag(T))
fwd = tmpl(["T"], [RRef(T)], Tag(T))
crf = tmpl(["T"], [LRef(c(T))], Tag(T))
for what, t, tgt, want in [("a = take", take, Func(Tag(I), (I,)), I), ("b = &take", take, Func(Tag(L), (L,)), L),
                           ("c = fwd, int&", fwd, Func(Tag(LRef(I)), (LRef(I),)), LRef(I)),
                           ("d = fwd, int&&", fwd, Func(Tag(I), (RRef(I),)), I),
                           ("e = cref", crf, Func(Tag(I), (LRef(cI),)), I)]:
    r = deduce_target(t, tgt)
    record(tn, what, isinstance(r, Deduced) and r.args.get("T") == want, str(r))
r = deduce_target(take, Func(VOID, (I,)))
record("context-address-mismatch", "void(*)(int) = take", isinstance(r, Failure), str(r))

# context-conversion* (DED-31): the component has no model of conversion function templates.
skip("context-conversion", 6, "DED-31 is not modeled")
skip("context-conversion-differences", 2, "DED-31 is not modeled")
skip("context-conversion-exact", 2, "DED-31 is not modeled")
skip("context-conversion-qualification-place", 2, "DED-31 is not modeled")
skip("context-conversion-reference-qualifiers", 5, "DED-31 is not modeled")

# ---------------------------------------------------------------- declaration-* (DED-32, DED-33) through deduce_target
put1 = tmpl(["T"], [T, I])
put2 = tmpl(["T"], [I, T])
tgt = Func(VOID, (I, I))
for name in ("declaration-ambiguous", "declaration-ambiguous-instantiation"):
    m = [isinstance(deduce_target(t, tgt), Deduced) for t in (put1, put2)]
    o12, o21 = at_least_as_specialized(put1, put2, Env()), at_least_as_specialized(put2, put1, Env())
    record(name, "two templates match, not ordered: reject", all(m) and o12 == o21, f"match {m}, order {o12} {o21}")
w1 = tmpl(["T"], [T], I)
w2 = tmpl(["T"], [Ptr(T)], I)
for name, tg in (("declaration-specialization", Func(I, (Ptr(I),))), ("declaration-specialization", Func(I, (Ptr(L),))),
                 ("declaration-specialization-friend", Func(I, (Ptr(Cls("Box")),)))):
    m = [isinstance(deduce_target(t, tg), Deduced) for t in (w1, w2)]
    o = at_least_as_specialized(w2, w1, Env()) and not at_least_as_specialized(w1, w2, Env())
    record(name, f"{tg!r} goes to which(T*)", all(m) and o, f"match {m}, order {o}")
picks("declaration-specialization", "which(char*) == 2", [w1, w2], [pv(Ptr(CH))], 1)

# ---------------------------------------------------------------- default-argument-*
rejects("default-argument-not-deduced", "fill()", [tmpl(["T"], [FP(T, default=True)])], [], rule="DED-35")
mix = tmpl(["T"], [T, FP(T, default=True)])
chk("default-argument-passed", "mix(1)", mix, [pv(I)], {"T": I})
chk("default-argument-passed", "mix(1, 2)", mix, [pv(I), pv(I)], {"T": I})

# ---------------------------------------------------------------- difference-*
e = Env()
e.class_templates["Base"] = lambda a: ClassDef(bases=[spec("Base", L)] if a == (I,) else [])
e.classes["Derived"] = ClassDef(bases=[spec("Base", I)])
e.classes["Lower"] = ClassDef(bases=[Cls("Derived")])
up = tmpl(["T"], [LRef(spec("Base", T))])
up_ptr = tmpl(["T"], [Ptr(c(spec("Base", T)))])
tn = "difference-derived"
chk(tn, "up(d)", up, [lv(Cls("Derived"))], {"T": I}, env=e)
chk(tn, "up(l)", up, [lv(Cls("Lower"))], {"T": I}, env=e)
chk(tn, "up_ptr(&d)", up_ptr, [pv(Ptr(Cls("Derived")))], {"T": I}, env=e)
chk("difference-exact-first", "up(Base<int>)", up, [lv(spec("Base", I))], {"T": I}, env=e)
e2 = Env()
e2.class_templates["Rec"] = lambda a: ClassDef(bases=[spec("Rec", *a[1:])] if a else [])
walk = tmpl([TP("A", pack=True)], [LRef(c(spec("Rec", Expand(A_))))])
chk("difference-derived-exact-first", "walk(Rec<int, char>)", walk, [pv(spec("Rec", I, CH))], {"A": (I, CH)}, env=e2)
e3 = Env()
e3.class_templates["Base"] = lambda a: ClassDef(bases=[spec("Base", CH)] if a == (I,) else [])
e3.classes["Side"] = ClassDef(bases=[spec("Base", CH)])
e3.classes["Join"] = ClassDef(bases=[spec("Base", I), Cls("Side")])
chk("difference-derived-nearest", "up(j) T = int", up, [lv(Cls("Join"))], {"T": I}, env=e3)
find = tmpl([TP("TT", "template"), "T"], [LRef(Cls(TTParam("TT"), (T,), is_template=True))])
chk("difference-derived-template-template", "find(d)", find, [lv(Cls("Derived"))], {"TT": TmplName("Base"), "T": I}, env=e)
e4 = Env(); e4.class_templates["Base"] = lambda a: ClassDef()
e4.classes["Both"] = ClassDef(bases=[spec("Base", I), spec("Base", CH)])
rejects("difference-derived-two-bases", "up(b)", [up], [lv(Cls("Both"))], env=e4, rule="DED-35")
tn = "difference-qualification"
deep = tmpl(["T"], [Ptr(Ptr(c(T), cvs(C)))])
chk(tn, "deep(int**)", deep, [lv(Ptr(Ptr(I)))], {"T": I})
call = tmpl(["R"], [Ptr(Func(R, ()))])
chk(tn, "call(give noexcept)", call, [lv(Func(I, (), False, True))], {"R": I})
e5 = Env(); e5.classes["Owner"] = ClassDef()
field_ = tmpl(["T", "C"], [MemPtr(TParam("C"), c(T))])
chk(tn, "field(&Owner::value)", field_, [pv(MemPtr(Cls("Owner"), I))], {"T": I}, env=e5)

# ---------------------------------------------------------------- explicit-*
pk_plain = plain([I])
pk_t = tmpl(["T"], [T])
tn = "explicit-empty-list"
picks(tn, "pick(1) plain", [pk_plain, pk_t], [pv(I)], 0)
picks(tn, "pick<>(1) template", [pk_plain, pk_t], [pv(I)], 1, explicit=())
# resolve() cannot tell `pick<>(1)` from `pick(1)`: an empty explicit list is the same as none.
tn = "explicit-empty-list-default"
show_t = tmpl([TP("T", default=I)], [L])
picks(tn, "show(1) plain", [pk_plain, show_t], [pv(I)], 0)
picks(tn, "show<>(1) template", [pk_plain, show_t], [pv(I)], 1, explicit=())
mixe = tmpl(["T", "U"], [T, U, T], U)
chk("explicit-fixed-later-pair", "mix<long>(1, 2.0f, 'c') U = float", mixe, [pv(I), pv(F_), pv(CH)], {"T": L, "U": F_}, explicit=[L])
chk("explicit-fixed-later-pair", "value 102", mixe, [pv(I), pv(F_), pv(CH)], {"U": F_}, explicit=[L])
e = Env(); e.classes["Meter"] = ClassDef()
add = tmpl(["T"], [T, T], T)
rejects("explicit-fixed-not-deduced", "add(Meter{}, 1)", [add], [pv(Cls("Meter")), pv(I)], env=e, rule="DED-35")
tn = "explicit-fixed-parameter-converts"
chk(tn, "add<long>(1, 2L)", add, [pv(I), pv(L)], {"T": L}, explicit=[L])
chk(tn, "add<long>(1, 'a')", add, [pv(I), pv(CH)], {"T": L}, explicit=[L])
skip(tn, 1, "add<int>(Meter{}, 1) needs a conversion function; the conversion model has none")
tn = "explicit-invalid-removes"
size1 = tmpl([TP("N", "const", ctype=I)], [])
size2 = tmpl(["T"], [])
picks(tn, "size<int>() is Two", [size1, size2], [], 1, explicit=[I])
picks(tn, "size<3>() is One", [size1, size2], [], 0, explicit=[Val(3, I)])
code1 = tmpl([TP("C", "const", ctype=Prim("unsigned char"))], [])
code2 = tmpl([TP("C", "const", ctype=I)], [])
picks(tn, "code<300>() is Two", [code1, code2], [], 1, explicit=[Val(300, I)])
cnt1 = tmpl(["T"], [T])
cnt2 = tmpl(["T", "U"], [T])
picks(tn, "count<int, int>(1) is Two", [cnt1, cnt2], [pv(I)], 1, explicit=[I, I])
tn = "explicit-invalid-template-template"
wr1 = tmpl([TP("W", "template")], [])
wr2 = tmpl(["T"], [])
picks(tn, "wrap<Box>() is One", [wr1, wr2], [], 0, explicit=[TmplName("Box")])
picks(tn, "wrap<int>() is Two", [wr1, wr2], [], 1, explicit=[I])
tn = "explicit-leading"
conv = tmpl(["To", "From"], [TParam("From")])
chk(tn, "convert<long>(1)", conv, [pv(I)], {"To": L, "From": I}, explicit=[L])
chk(tn, "convert<long, char>(1)", conv, [pv(I)], {"To": L, "From": CH}, explicit=[L, CH])
allt = tmpl([TP("Ts", pack=True)], [FP(Ts, pack=True)])
chk(tn, "all<long>(1, 'c')", allt, [pv(I), pv(CH)], {"Ts": (L, CH)}, explicit=[L])
tn = "explicit-pack-then-deduced"
chk(tn, "all<long, char>(1, 2, 3.0)", allt, [pv(I), pv(I), pv(D)], {"Ts": (L, CH, D)}, explicit=[L, CH])
make = tmpl(["R", TP("Ts", pack=True)], [FP(Ts, pack=True)])
chk(tn, "make<int>('a', 1L)", make, [pv(CH), pv(L)], {"R": I, "Ts": (CH, L)}, explicit=[I])

# ---------------------------------------------------------------- forwarding-*
tn = "forwarding-class-parameter-not"
p1, p2 = plain([RRef(I)]), plain([LRef(cI)])
picks(tn, "b.put(n) is Two", [p1, p2], [lv(I)], 1)
picks(tn, "b.put(1) is One", [p1, p2], [pv(I)], 0)
rejects("forwarding-const-not", "take(n)", [tmpl(["T"], [RRef(c(T))])], [lv(I)], rule="DED-35")
nest = tmpl(["T"], [Ptr(Func(VOID, (RRef(T),)))])
rejects("forwarding-nested-not", "take(sink(int&))", [nest], [lv(Func(VOID, (LRef(I),)))], rule="DED-35")
chk("forwarding-nested-rvalue", "take(sink(int&&))", nest, [lv(Func(VOID, (RRef(I),)))], {"T": I})
tn = "forwarding-reference"
chk(tn, "fwd(n)", fwd, [lv(I)], {"T": LRef(I)})
chk(tn, "fwd(c)", fwd, [lv(cI)], {"T": LRef(cI)})
chk(tn, "fwd(1)", fwd, [pv(I)], {"T": I})
chk(tn, "fwd(xvalue)", fwd, [xv(I)], {"T": I})
r = deduce(fwd, [lv(I)])
record(tn, "binds_lvalue(n): parameter is int&", isinstance(r, Deduced) and r.ptypes == [LRef(I)], str(r))

# losing-candidate-body-not-instantiated
e = Env(); e.classes["Circle"] = ClassDef(); e.classes["Rect"] = ClassDef()
gen = tmpl(["T"], [LRef(c(T))], I, deduced_ret=True, body_error_if=lambda b: b["T"] == Cls("Circle"))
pl = plain([LRef(c(Cls("Circle")))], ret=I)
picks("losing-candidate-body-not-instantiated", "area(Circle{1}) plain", [pl, gen], [pv(Cls("Circle"))], 0, env=e)
picks("losing-candidate-body-not-instantiated", "area(Rect{2}) generic", [pl, gen], [pv(Cls("Rect"))], 1, env=e)

# ---------------------------------------------------------------- match-*
e = Env()
for nm, ct in (("Wide", L), ("Letter", CH), ("Fixed", I)):
    e.class_templates[nm] = lambda a: ClassDef()
    e.ctemplate_params[nm] = [TP("N", "const", ctype=ct)]
e.class_templates["Value"] = lambda a: ClassDef(); e.ctemplate_params["Value"] = [TP("V", "const", ctype=Prim("auto"))]
AUTO = Prim("auto")
tn = "match-argument-type"
at = tmpl([TP("N", "const", ctype=AUTO)], [spec("Wide", CParam("N"))])
r = deduce(at, [pv(spec("Wide", Val(4, L)))], (), e)
record(tn, "argument_type(Wide<4>) is long", isinstance(r, Deduced) and r.args["N"].type == L, str(r))
at2 = tmpl([TP("C", "const", ctype=AUTO)], [spec("Letter", CParam("C"))])
r = deduce(at2, [pv(spec("Letter", Val("x", CH)))], (), e)
record(tn, "argument_type(Letter<'x'>) is char", isinstance(r, Deduced) and r.args["C"].type == CH, str(r))
bt = tmpl([TP("N", "const", ctype=AUTO)], [LRef(Arr(I, CParam("N")))])
r = deduce(bt, [lv(Arr(I, 3))])
record("match-bound-type", "bound_type(arr) is the size type", isinstance(r, Deduced) and r.args["N"].type == SIZE_T, str(r))
cnt = tmpl(["T", TP("N", "const", ctype=I)], [LRef(Arr(T, CParam("N")))], I)
tn = "match-constant-parameters"
chk(tn, "count(arr) == 3", cnt, [lv(Arr(I, 3))], {"N": Val(3, I)})
chk(tn, "count(grid) == 2", cnt, [lv(Arr(Arr(I, 5), 2))], {"N": Val(2, I)})
chk(tn, "fixed(Fixed<4>) == 4", tmpl([TP("N", "const", ctype=I)], [spec("Fixed", CParam("N"))]), [pv(spec("Fixed", Val(4, I)))], {"N": Val(4, I)}, env=e)
chk(tn, "value_type(Value<'c'>) T = char", tmpl(["T", TP("V", "const", ctype=T)], [spec("Value", CParam("V"))]), [pv(spec("Value", Val("c", CH)))], {"T": CH}, env=e)
e.ctemplate_params["Wide2"] = [TP("V", "const", ctype=AUTO)]; e.class_templates["Wide2"] = lambda a: ClassDef()
kind = tmpl([TP("P", "const", ctype=Ptr(AUTO))], [spec("Wide2", CParam("P"))], I)
picks("match-constant-placeholder-pattern", "kind(Wide<&n>) == 1", [kind, plain([], True, I)], [pv(spec("Wide2", Val("&n", Ptr(I))))], 0, env=e)
picks("match-constant-placeholder-pattern", "kind(Wide<5>) == 2", [kind, plain([], True, I)], [pv(spec("Wide2", Val(5, I)))], 1, env=e)
r = deduce(at, [pv(spec("Wide", Val(5, L)))], (), e)
record("match-constant-placeholder-type", "width(Wide<5>) is long", isinstance(r, Deduced) and r.args["N"].type == L, str(r))
rejects("match-constant-type-differs", "take(Fixed<4>)", [tmpl([TP("N", "const", ctype=L)], [spec("Fixed", CParam("N"))])], [pv(spec("Fixed", Val(4, I)))], env=e, rule="DED-35")
chk("match-constant-type-same", "take(Fixed<4>) == 4", tmpl([TP("N", "const", ctype=cI)], [spec("Fixed", CParam("N"))], I), [pv(spec("Fixed", Val(4, I)))], {"N": Val(4, I)}, env=e)
nt = tmpl([TP("B", "const", ctype=BOOL)], [Ptr(Func(VOID, (), False, CParam("B")))], BOOL)
chk("match-noexcept", "nothrow(quiet)", nt, [lv(Func(VOID, (), False, True))], {"B": Val(True, BOOL)})
chk("match-noexcept", "!nothrow(loud)", nt, [lv(Func(VOID, ()))], {"B": Val(False, BOOL)})
rejects("match-shape-mismatch", "point(1)", [tmpl(["T"], [Ptr(T)])], [pv(I)], rule="DED-35")
e = Env()
e.class_templates["Box"] = lambda a: ClassDef(); e.class_templates["Hold"] = lambda a: ClassDef()
e.ctemplate_params["Hold"] = [TP("TT", "template")]
remake = tmpl([TP("TT", "template")], [Cls(TTParam("TT"), (I,), is_template=True)], Cls(TTParam("TT"), (CH,), is_template=True))
chk_ret("match-template-template", "remake(Box<int>) is Box<char>", remake, [pv(spec("Box", I))], spec("Box", CH), env=e)
unwrap = tmpl([TP("TT", "template")], [spec("Hold", TTParam("TT"))], Cls(TTParam("TT"), (CH,), is_template=True))
chk_ret("match-template-template", "unwrap(Hold<Box>) is Box<char>", unwrap, [pv(spec("Hold", TmplName("Box")))], spec("Box", CH), env=e)

tn = "match-compound-types"
e = Env()
for nm in ("Box", "Pair", "Tag"):
    e.class_templates[nm] = lambda a: ClassDef()
e.classes["Owner"] = ClassDef()
Ow = Cls("Owner")
chk(tn, "ptrptr(int**)", tmpl(["T"], [Ptr(Ptr(T))]), [lv(Ptr(Ptr(I)))], {"T": I}, env=e)
member = tmpl(["T", "C"], [MemPtr(TParam("C"), T)])
chk(tn, "member(&Owner::field)", member, [pv(MemPtr(Ow, I))], {"T": I, "C": Ow}, env=e)
chk(tn, "member(&Owner::method)", member, [pv(MemPtr(Ow, Func(CH, (L,))))], {"T": Func(CH, (L,)), "C": Ow}, env=e)
chk(tn, "fnptr(&twice)", tmpl(["R", "A"], [Ptr(Func(R, (A_,)))]), [pv(Ptr(Func(CH, (CH,))))], {"R": CH, "A": CH}, env=e)
chk(tn, "boxed(Box<int>)", tmpl(["T"], [spec("Box", T)]), [pv(spec("Box", I))], {"T": I}, env=e)
chk(tn, "boxptr(Box<char*>)", tmpl(["T"], [LRef(c(spec("Box", Ptr(T))))]), [pv(spec("Box", Ptr(CH)))], {"T": CH}, env=e)
chk(tn, "paired(Pair<int, Box<char>>)", tmpl(["T", "U"], [spec("Pair", T, spec("Box", U))]), [pv(spec("Pair", I, spec("Box", CH)))], {"T": I, "U": CH}, env=e)
chk(tn, "any(Box<long>)", tmpl([TP("B", "template"), "T"], [Cls(TTParam("B"), (T,), is_template=True)]), [pv(spec("Box", L))], {"B": TmplName("Box"), "T": L}, env=e)
chk(tn, "whole(both)", tmpl(["R", TP("A", pack=True)], [LRef(Func(R, (Expand(A_),)))]), [lv(Func(L, (I, CH)))], {"R": L, "A": (I, CH)}, env=e)

# ---------------------------------------------------------------- nondeduced-*
e = Env(); e.aliases["Id"] = lambda a: I
e.classes["Meters"] = ClassDef(complete=False)
tg = tmpl(["Tag"], [AliasT("Id", (TParam("Tag"),)), Ptr(TParam("Tag"))], I)
chk("nondeduced-alias-unused-other-position", "tagged(1, where)", tg, [pv(I), lv(Ptr(Cls("Meters")))], {"Tag": Cls("Meters")}, env=e)
nm = tmpl(["Tag"], [AliasT("Id", (TParam("Tag"),))], I)
chk("nondeduced-alias-unused-other-position", "named<Meters>(1)", nm, [pv(I)], {"Tag": Cls("Meters")}, explicit=[Cls("Meters")], env=e)
rejects("nondeduced-alias-unused-parameter", "tagged(1)", [nm], [pv(I)], env=e, rule="DED-35")
fill = tmpl([TP("N", "const", ctype=I)], [Arr(I, CParam("N"))])
rejects("nondeduced-array-by-value", "fill(arr)", [fill], [lv(Arr(I, 3))], rule="DED-35")
dt = lambda b, env: b["T"]
dt.names = ("T",)
rejects("nondeduced-decltype", "typed(1)", [tmpl(["T"], [Decltype(dt, "decltype(T{})")])], [pv(I)], rule="DED-35")
e = Env(); e.class_templates["Id"] = lambda a: ClassDef(members={"type": (a[0], "public")})
only = tmpl(["T"], [Qualified(spec("Id", T), "type")])
chk("nondeduced-explicit", "only<long>(1)", only, [pv(I)], {"T": L}, explicit=[L], env=e)
chk("nondeduced-explicit", "fill<3>(arr)", fill, [lv(Arr(I, 3))], {"N": Val(3, I)}, explicit=[Val(3, I)])
e.class_templates["Fixed"] = lambda a: ClassDef(); e.ctemplate_params["Fixed"] = [TP("N", "const", ctype=I)]
np1 = ValExpr(lambda b: Val(b["N"].value + 1, I), "N + 1"); np1.fn.names = ("N",)
nxt = tmpl([TP("N", "const", ctype=I)], [spec("Fixed", np1)])
rejects("nondeduced-expression", "next(Fixed<3>)", [nxt], [pv(spec("Fixed", Val(3, I)))], env=e, rule="DED-35")
e.class_templates["Outer"] = lambda a: ClassDef(members={"In": (None, "public")})
inn = Decltype(lambda b, env: spec("Outer::In", b["U"]), "typename Outer<T>::template In<U>"); inn.fn.names = ("T", "U")
tk = tmpl(["T", "U"], [T, inn])
rejects("nondeduced-member-template-arguments", "take(1, Outer<int>::In<char>{})", [tk], [pv(I), pv(spec("Outer::In", CH))], env=e, rule="DED-35")
chk("nondeduced-member-template-explicit", "take<int, char>(...)", tk, [pv(I), pv(spec("Outer::In", CH))], {"U": CH}, explicit=[I, CH], env=e)
first = tmpl(["T"], [Qualified(spec("Id", T), "type"), T])
chk("nondeduced-qualified-name", "first(1L, 2) T = int", first, [pv(L), pv(I)], {"T": I}, env=e)
nx2 = tmpl([TP("N", "const", ctype=I)], [spec("Fixed", np1), spec("Fixed", CParam("N"))], I)
chk("nondeduced-qualified-name", "next(Fixed<4>, Fixed<3>) == 3", nx2, [pv(spec("Fixed", Val(4, I))), pv(spec("Fixed", Val(3, I)))], {"N": Val(3, I)}, env=e)
rejects("nondeduced-qualified-only", "only(1)", [only], [pv(I)], env=e, rule="DED-35")

# ---------------------------------------------------------------- overload-set-*
fset = OverloadArg([Func(VOID, (I,)), Func(VOID, (D,))])
chk("overload-set-default", "take(f) F = void(*)(int)", tmpl([TP("F", default=Ptr(Func(VOID, (I,))))], [TParam("F")], I), [fset], {"F": Ptr(Func(VOID, (I,)))})
act = OverloadArg([Func(I, (I,)), Func(I, (L,))])
run = tmpl(["T"], [T, Ptr(Func(I, (T,)))], I)
chk("overload-set-later", "run(1, act) == 1", run, [pv(I), act], {"T": I})
chk("overload-set-later", "run(1L, act) == 2", run, [pv(L), act], {"T": L})
applyi = tmpl(["T"], [Ptr(Func(T, (I,)))])
chk("overload-set-one-match", "apply(twice) T = int", applyi, [OverloadArg([Func(I, (I,)), Func(L, (L,))])], {"T": I})
rejects("overload-set-template", "apply(make)", [applyi], [OverloadArg([Func(I, (I,))], has_template=True)], rule="DED-35")
rejects("overload-set-two-match", "apply(square)", [tmpl(["T"], [Ptr(Func(T, (T,)))])], [OverloadArg([Func(I, (I,)), Func(L, (L,))])], rule="DED-35")

# ---------------------------------------------------------------- pack-*
e = Env(); e.class_templates["List"] = lambda a: ClassDef()
List = lambda *a: spec("List", *a)
tn = "pack-class-arguments"
allL = tmpl([TP("A", pack=True)], [List(Expand(A_))])
rest = tmpl(["H", TP("A", pack=True)], [List(TParam("H"), Expand(A_))])
chk(tn, "all(List<>)", allL, [pv(List())], {"A": ()}, env=e)
chk(tn, "all(List<int, char>)", allL, [pv(List(I, CH))], {"A": (I, CH)}, env=e)
chk(tn, "rest(List<int>)", rest, [pv(List(I))], {"A": ()}, env=e)
chk(tn, "rest(List<int, char, long>)", rest, [pv(List(I, CH, L))], {"A": (CH, L)}, env=e)
ptrs = tmpl([TP("A", pack=True)], [List(Expand(Ptr(A_)))])
chk("pack-class-arguments-pointer-pattern", "ptrs(List<float*>)", ptrs, [pv(List(Ptr(F_)))], {"A": (F_,)}, env=e)
chk("pack-class-arguments-pointer-pattern", "ptrs(List<int*, const char*>)", ptrs, [pv(List(Ptr(I), Ptr(c(CH))))], {"A": (I, c(CH))}, env=e)
refs = tmpl([TP("A", pack=True)], [List(Expand(LRef(A_)))])
chk("pack-class-arguments-reference-pattern", "refs(List<float&>)", refs, [pv(List(LRef(F_)))], {"A": (F_,)}, env=e)
chk("pack-class-arguments-reference-pattern", "refs(List<int&, char&>)", refs, [pv(List(LRef(I), LRef(CH)))], {"A": (I, CH)}, env=e)
params = tmpl([TP("A", pack=True)], [Ptr(Func(VOID, (Expand(A_),)))])
chk("pack-function-type", "params(two)", params, [lv(Func(VOID, (I, CH)))], {"A": (I, CH)})
chk("pack-function-type", "params(none)", params, [lv(Func(VOID, ()))], {"A": ()})
lead = tmpl([TP("Ts", pack=True)], [FP(Ts, pack=True), I])
rejects("pack-not-last-call", "lead(1, 2)", [lead], [pv(I), pv(I)], rule="DED-35")
chk("pack-not-last", "lead(1)", lead, [pv(I)], {"Ts": ()})
chk("pack-not-last", "lead<char>('a', 1)", lead, [pv(CH), pv(I)], {"Ts": (CH,)}, explicit=[CH])
tka = tmpl([TP("A", pack=True)], [FP(A_, pack=True), List(Expand(A_))])
rejects("pack-not-last-other-position", "take(1, 2, List<int,int>)", [tka], [pv(I), pv(I), pv(List(I, I))], env=e, rule="DED-35")
tn = "pack-trailing"
refsT = tmpl([TP("Ts", pack=True)], [FP(RRef(Ts), pack=True)])
unpack = tmpl([TP("Ts", pack=True)], [List(Expand(Ts))])
chk(tn, "all(1, 'c', arr)", allt, [pv(I), pv(CH), lv(Arr(I, 3))], {"Ts": (I, CH, Ptr(I))})
chk(tn, "all()", allt, [], {"Ts": ()})
chk(tn, "refs(n, 1)", refsT, [lv(I), pv(I)], {"Ts": (LRef(I), I)})
chk(tn, "unpack(List<int, char>)", unpack, [pv(List(I, CH))], {"Ts": (I, CH)}, env=e)

# ---------------------------------------------------------------- placeholder-*
X = TParam("auto1")
tn = "placeholder-parameter-after-pack"
cntp = tmpl([TP("T", pack=True), "auto1"], [X], I)
chk(tn, "count<int, long>(1) == 2", cntp, [pv(I)], {"T": (I, L), "auto1": I}, explicit=[I, L])
chk(tn, "count<int, long>(1L) == 12", cntp, [pv(L)], {"T": (I, L), "auto1": L}, explicit=[I, L])
sil = tmpl(["T", "auto1"], [X], BOOL)
chk(tn, "second_is_long<int, long>(1)", sil, [pv(I)], {"auto1": L}, explicit=[I, L])
tn = "placeholder-parameter"
tw = tmpl(["auto1"], [X])
chk(tn, "twice(2)", tw, [pv(I)], {"auto1": I})
chk(tn, "twice(1.5) double", tw, [pv(D)], {"auto1": D})
mx = tmpl(["T", "auto1"], [T, X])
chk(tn, "mix<int, char>(1, 'c')", mx, [pv(I), pv(CH)], {"T": I, "auto1": CH}, explicit=[I, CH])
chk(tn, "mix<long>(1, 'c')", mx, [pv(I), pv(CH)], {"T": L, "auto1": CH}, explicit=[L])
X2 = TParam("auto2")
both = tmpl(["auto1", "auto2"], [X, LRef(c(X2))])
r = deduce(both, [pv(I), pv(L)], [I, L])
record(tn, "both<int, long>(1, 2L)", isinstance(r, Deduced) and r.ptypes == [I, LRef(c(L))], str(r))
tn = "placeholder-parameter-pointer-pack"
chk(tn, "deref(&n)", tmpl(["auto1"], [Ptr(c(X))], I), [pv(Ptr(cI))], {"auto1": I})
cp = tmpl([TP("auto1", pack=True)], [FP(X, pack=True)], I)
chk(tn, "count() == 0", cp, [], {"auto1": ()})
chk(tn, "count(1, 'c', 2.0) == 3", cp, [pv(I), pv(CH), pv(D)], {"auto1": (I, CH, D)})
fst = tmpl(["auto1", TP("auto2", pack=True)], [Ptr(X), FP(X2, pack=True)])
r = deduce(fst, [Arg(Prim("decltype(nullptr)")), pv(CH)], [I])
record(tn, "first<int>(nullptr, 'c')", isinstance(r, Deduced) and r.viable and r.ptypes == [Ptr(I), CH], str(r))

# ---------------------------------------------------------------- placement-* (DED-39): not modeled
skip("placement-deallocation-match", 1, "DED-39 is not modeled")
skip("placement-deallocation-ordered", 1, "DED-39 is not modeled")
skip("placement-deallocation-ordered-instantiation", 1, "DED-39 is not modeled")

# ---------------------------------------------------------------- specialization-*
kp = tmpl(["T"], [T])
r1, r2 = deduce(kp, [], [cI]), deduce(kp, [], [I])
record("specialization-distinct-functions", "&keep<const int> is void(*)(int)", isinstance(r1, Deduced) and r1.ptypes == [I], str(r1))
skip("specialization-distinct-functions", 1, "addresses of two specializations (DED-38) are outside the model")
idt = tmpl(["T"], [I], I)
r = deduce(idt, [], [CH])
record("specialization-distinct-unused-parameter", "&id<char> is int(*)(int)", isinstance(r, Deduced) and r.ptypes == [I], str(r))
skip("specialization-distinct-unused-parameter", 1, "addresses of two specializations (DED-38) are outside the model")

# ---------------------------------------------------------------- substitution-*
e = Env()
e.classes["Secret"] = ClassDef(members={"type": (I, "private")})
e.classes["Open"] = ClassDef(members={"type": (I, "public")})
get = tmpl(["T"], [T, FP(Ptr(Qualified(T, "type")), default=True)])
picks("substitution-access", "get(Open{}) is One", [get, ELL], [pv(Cls("Open"))], 0, env=e)
picks("substitution-access", "get(Secret{}) is Two", [get, ELL], [pv(Cls("Secret"))], 1, env=e)
e.classes["Locked"] = ClassDef(members={"run": (Func(VOID, ()), "private")})
e.classes["Free"] = ClassDef(members={"run": (Func(VOID, ()), "public")})
def run_call(b, env):
    d = env.classdef(b["T"]) if isinstance(b["T"], Cls) else None
    if d is None or "run" not in d.members:
        raise SubstFail("DED-24", "no run")
    if d.members["run"][1] != "public":
        raise SubstFail("DED-26", "run is private")
    return Cls("One")
run_call.names = ("T",)
cl = tmpl(["T"], [T], Decltype(run_call), trailing=True)
picks("substitution-access-member-function", "call(Free{}) is One", [cl, ELL], [pv(Cls("Free"))], 0, env=e)
picks("substitution-access-member-function", "call(Locked{}) is Two", [cl, ELL], [pv(Cls("Locked"))], 1, env=e)
r = deduce(kp, [], [cI])
record("substitution-adjusted-type", "&take<const int> is void(*)(int)", isinstance(r, Deduced) and r.ptypes == [I], str(r))
r = deduce(kp, [], [Arr(I, 3)])
record("substitution-adjusted-type", "&take<int[3]> is void(*)(int*)", isinstance(r, Deduced) and r.ptypes == [Ptr(I)], str(r))
def field_call(b, env):
    raise Hard("DED-29", "the body of field<int> has an error")
field_call.names = ("T",)
pick_t = tmpl(["T"], [T], Decltype(field_call), trailing=True)
rejects("substitution-deduced-result-body", "pick(1)", [pick_t, ELL], [pv(I)], rule="DED-29")
e = Env()
e.class_templates["Bad"] = lambda a: ClassDef(body_error=True)
e.class_templates["Hard"] = lambda a: ClassDef(body_error=True)
pd = tmpl(["T", TP("U", default=Qualified(spec("Bad", T), "type"))], [Ptr(I), T])
rejects("substitution-default-before-check", "pick(1, 1)", [pd, ELL], [pv(I), pv(I)], env=e, rule="DED-29")
def expl(b):
    if not isinstance(b["T"], Cls):
        raise SubstFail("DED-24", "int::value")
ctor = tmpl(["T"], [T], explicit_cond=expl)
picks("substitution-explicit-specifier", "Pick(1).which == 2", [ctor, ELL], [pv(I)], 1)
e2 = Env(); e2.class_templates["Box"] = lambda a: ClassDef()
def depth(b):
    if b["D"].value > 512:  # the limit of CE-10, a model value
        raise SubstFail("DED-24", "evaluation limit")
    return Val(b["D"].value, I)
depth.names = ("D",)
pk1 = tmpl([TP("D", "const", ctype=I)], [Ptr(spec("Box", ValExpr(depth)))], I)
pk2 = tmpl([TP("D", "const", ctype=I)], [], I, variadic=True)
NUL = Arg(Prim("decltype(nullptr)"), "prvalue", True)
picks("substitution-failure-evaluation-limit", "pick<10>(nullptr) == 1", [pk1, pk2], [NUL], 0, explicit=[Val(10, I)], env=e2)
picks("substitution-failure-evaluation-limit", "pick<100000>(nullptr) == 2", [pk1, pk2], [NUL], 1, explicit=[Val(100000, I)], env=e2)
def divide(b):
    if b["B"].value == 0:
        raise SubstFail("DED-24", "division by zero")
    return Val(1 // b["B"].value, I)
divide.names = ("B",)
pd1 = tmpl([TP("B", "const", ctype=I)], [Ptr(spec("Box", ValExpr(divide)))], I)
pd2 = tmpl([TP("B", "const", ctype=I)], [], I, variadic=True)
picks("substitution-failure-undefined-operation", "pick<1>(nullptr) == 1", [pd1, pd2], [NUL], 0, explicit=[Val(1, I)], env=e2)
picks("substitution-failure-undefined-operation", "pick<0>(nullptr) == 2", [pd1, pd2], [NUL], 1, explicit=[Val(0, I)], env=e2)
ZERO = Arg(I, "prvalue", True)
tn = "substitution-failure-forms"
ell = lambda: tmpl(["T"], [], variadic=True)
zero_sz = ValExpr(lambda b: Val(0, SIZE_T), "sizeof(T) - sizeof(T)"); zero_sz.fn.names = ("T",)
forms = [("pointer<int&>", [Ptr(T)], LRef(I)), ("reference<void>", [LRef(T)], VOID),
         ("array<int&>", [Ptr(Arr(T, 1))], LRef(I)), ("member<int>", [MemPtr(T, I)], I),
         ("result<int[2]>", [Ptr(Func(T, ()))], Arr(I, 2)), ("empty<int>", [Ptr(Arr(CH, zero_sz))], I),
         ("nested<int>", [Qualified(T, "type")], I)]
for what, ps, ex in forms:
    picks(tn, what + "(0) is Two", [tmpl(["T"], ps), ell()], [ZERO], 1, explicit=[ex])
tn = "substitution-failure-negative-bound-computed"
nm2 = ValExpr(lambda b: Val(b["N"].value - 2, I), "N - 2"); nm2.fn.names = ("N",)
sh1 = tmpl([TP("N", "const", ctype=I)], [LRef(Arr(CH, nm2))])
sh2 = tmpl([TP("N", "const", ctype=I)], [], variadic=True)
picks(tn, "shorter<1>(0) is Two", [sh1, sh2], [ZERO], 1, explicit=[Val(1, I)])
picks(tn, "shorter<2>(0) is Two", [sh1, sh2], [ZERO], 1, explicit=[Val(2, I)])
tn = "substitution-failure-negative-bound"
br1 = tmpl([TP("N", "const", ctype=I)], [LRef(Arr(CH, CParam("N")))])
br2 = tmpl([TP("N", "const", ctype=I)], [], variadic=True)
bp1 = tmpl([TP("N", "const", ctype=I)], [Ptr(Arr(CH, CParam("N")))])
bp2 = tmpl([TP("N", "const", ctype=I)], [I])
picks(tn, "by_reference<-1>(0) is Two", [br1, br2], [ZERO], 1, explicit=[Val(-1, I)])
picks(tn, "by_reference<1>(0) is Two", [br1, br2], [ZERO], 1, explicit=[Val(1, I)])
picks(tn, "by_pointer<-1>(0) is Two", [bp1, bp2], [ZERO], 1, explicit=[Val(-1, I)])
picks("substitution-failure-negative-bound-ellipsis", "by_pointer<-1>(0) is Two", [bp1, br2], [ZERO], 1, explicit=[Val(-1, I)])
e = Env(); e.classes["Vec"] = ClassDef(members={"size_type": (I, "public")})
so = tmpl(["T"], [T, FP(Qualified(T, "size_type"), default=True)])
picks("substitution-failure-removes", "size_of(Vec{}) is One", [so, ELL], [pv(Cls("Vec"))], 0, env=e)
picks("substitution-failure-removes", "size_of(1) is Two", [so, ELL], [pv(I)], 1, env=e)
e = Env(); e.class_templates["Hard"] = lambda a: ClassDef(body_error=True)
ph = tmpl(["T"], [T, FP(Ptr(Qualified(spec("Hard", T), "type")), default=True)])
rejects("substitution-hard-error", "pick(1)", [ph, ELL], [pv(I)], env=e, rule="DED-29")
def lam_body(b, env):
    raise Hard("DED-29", "the lambda body has an error")
lam_body.names = ("T",)
pl = tmpl(["T"], [T, FP(Ptr(Decltype(lam_body)), default=True)])
rejects("substitution-lambda-body", "pick(1)", [pl, ELL], [pv(I)], rule="DED-29")
def lam_call(b, env):
    raise SubstFail("DED-24", "the lambda call has no viable function")
lam_call.names = ("T",)
plc = tmpl(["T"], [T], Decltype(lam_call), trailing=True)
picks("substitution-lambda-call-fails", "pick(1) is Two", [plc, ELL], [pv(I)], 1)
pld = tmpl(["T", TP("X", "const", ctype=I, default=ValExpr(lambda b: (_ for _ in ()).throw(Hard("DED-29", "lambda body"))))], [T])
pld2 = tmpl([TP("T", pack=True)], [FP(Ts if False else TParam("T"), pack=True)])
rejects("substitution-lambda-default", "pick(1)", [pld, pld2], [pv(I)], rule="DED-29")
actt = tmpl(["T"], [T], noexcept_error=lambda b: True)
rejects("substitution-noexcept-later", "act(1)", [actt, plain([L])], [pv(I)], rule="DED-27")
actt2 = tmpl(["T"], [T], I, noexcept_error=lambda b: True)
picks("substitution-noexcept-not-removed", "act(1) is act(int)", [actt2, plain([I], ret=L)], [pv(I)], 1)
e = Env(); e.classes["Plain"] = ClassDef(); e.class_templates["Hard"] = lambda a: ClassDef(body_error=True)
pn = tmpl(["T"], [T, Cls("Plain"), FP(Ptr(Qualified(spec("Hard", T), "type")), default=True)])
picks("substitution-nondependent-first", "pick(1, 2) is Two", [pn, plain([I, I])], [pv(I), pv(I)], 1, env=e)
po = tmpl(["T"], [T, Ptr(Qualified(T, "type")), FP(Ptr(Qualified(spec("Hard", T), "type")), default=True)])
picks("substitution-order", "pick(1, nullptr) is Two", [po, ELL], [pv(I), NUL], 1, env=e)
e.class_templates["Hard"] = lambda a: ClassDef(body_error=True, members={"type": (I, "public")})
por = tmpl(["T"], [T, FP(Ptr(Qualified(spec("Hard", T), "type")), default=True)], Qualified(T, "type"))
picks("substitution-order-return", "pick(1) is Two", [por, ELL], [pv(I)], 1, env=e)

# ---------------------------------------------------------------- grids
grid_dir = os.path.join(HERE, "..", "..", "..", "tests", "templates", "deduction", "grids")
tables = open(os.path.join(grid_dir, "tables.md")).read()


def table(name):
    sec = tables.split(f"## {name}\n")[1].split("\n## ")[0]
    rows = [l for l in sec.splitlines() if l.startswith("| ") and not l.startswith("| |")]
    head = [l for l in sec.splitlines() if l.startswith("| |")][0]
    cols = [x.strip() for x in head.strip("|").split("|")][1:]
    out = {}
    for l in rows:
        cells = [x.strip() for x in l.strip("|").split("|")]
        for col, v in zip(cols, cells[1:]):
            out[(cells[0], col)] = v
    return out


ct = table("call-types")
cols = {"T": T, "const T": c(T), "T lref": LRef(T), "const T lref": LRef(c(T)), "T rref": RRef(T),
        "T pointer": Ptr(T), "const T pointer": Ptr(c(T))}
fnc = Func(I, (CH,))
argsof = {"int lvalue": lv(I), "const int lvalue": lv(cI), "int prvalue": pv(I), "int xvalue": xv(I),
          "array": lv(Arr(I, 3)), "function": lv(fnc), "const int pointer": lv(Ptr(cI))}
wants = {"int": I, "int lref": LRef(I), "const int": cI, "const int lref": LRef(cI), "int pointer": Ptr(I),
         "int array": Arr(I, 3), "int array lref": LRef(Arr(I, 3)), "function": fnc,
         "function pointer": Ptr(fnc), "function lref": LRef(fnc), "const int pointer": Ptr(cI),
         "const int pointer lref": LRef(Ptr(cI))}
for (row, col), v in ct.items():
    a, w = row.split(" gives ")
    t = tmpl(["T"], [cols[col]])
    r = deduce(t, [argsof[a]])
    pred = "accept" if eq(r, {"T": wants[w]}) else "reject"
    record("grids/call-types", f"{row} / {col}", pred == v, f"table {v}, got {r}")

bt = table("braced")
bcols = {"T": tmpl(["T"], [T]), "initializer_list of T": lst, "const initializer_list of T lref": lst_ref,
         "T array of N rref": elem, "const T array of N lref": lenT,
         "const T array of 2 lref": tmpl(["T"], [LRef(Arr(c(T), 2))]),
         "const T array of M by N lref": area, "Box of T": tmpl(["T"], [spec("Box", T)]),
         "initializer_list of int and T": tmpl(["T"], [IL(I), FP(T, default=True)])}
brows = {"empty": Braced([]), "one int": Braced([pv(I)]), "two ints": Braced([pv(I)] * 2),
         "three ints": Braced([pv(I)] * 3), "int and long": Braced([pv(I), pv(L)]),
         "two chars": Braced([pv(CH)] * 2), "nested pairs": Braced([Braced([pv(I), pv(I)]), Braced([pv(I), pv(I)])])}
for (row, col), v in bt.items():
    r = resolve([bcols[col]], [brows[row]], (), env_basic())
    pred = "accept" if isinstance(r, Chosen) else "reject"
    record("grids/braced", f"{row} / {col}", pred == v, f"table {v}, got {r}")

# ---------------------------------------------------------------- other tests citing DED rules
e = Env()
e.class_templates["Owner"] = lambda a: ClassDef()
own = tmpl(["U"], [RRef(spec("Owner", U))])
chk("classes/constructors/return-owner-converts-to-base-owner", "Owner<Shape>(Owner<Square>&&) U = Square", own, [xv(spec("Owner", Cls("Square")))], {"U": Cls("Square")}, env=e)
chk("classes/constructors/return-owner-converts-to-base-owner", "from a prvalue", own, [pv(spec("Owner", Cls("Square")))], {"U": Cls("Square")}, env=e)
e = Env(); e.classes["lambda"] = ClassDef()
chk("classes/copy-move/assign-from-lambda-through-constructor-template", "Task(F f) F = closure", tmpl(["F"], [TParam("F")]), [pv(Cls("lambda"))], {"F": Cls("lambda")}, env=e)
e = Env(); e.classes["G"] = ClassDef()
G = Cls("G")
sf = tmpl(["Self"], [RRef(TParam("Self"))])
tn = "classes/members/explicit-object-deduced-forwarding"
r = [deduce(sf, [a], (), e) for a in (lv(G), lv(c(G)), pv(G))]
record(tn, "g, c, G{} give G&, const G&, G", [x.args.get("Self") for x in r] == [LRef(G), LRef(c(G)), G], str(r))
tn = "classes/members/explicit-object-deduced-constant"
gsf = tmpl(["Self"], [LRef(c(TParam("Self")))])
record(tn, "get(this const Self&) and at(this Self&&)", eq(deduce(gsf, [lv(G)], (), e), {"Self": G}) and eq(deduce(sf, [pv(G)], (), e), {"Self": G}), "")
sm = tmpl(["T"], [T, T], T)
chk("classes/operators/static-call-operator-template", "Sum{}(2, 3), (2L, 3L)", sm, [pv(I), pv(I)], {"T": I})
e = Env(); e.classes["Box"] = ClassDef()
chk("classes/operators/call-operator-template", "id(3), id(Box{4}), id(2.5)", val, [pv(Cls("Box"))], {"T": Cls("Box")}, env=e)
chk("declarations/initialization/list/braced-constructor-template", "M y{2}: V = int", val, [pv(I)], {"T": I})
for nm in ("throw-placement-function-and-template", "throw-placement-function-before-template-instantiation",
           "throw-placement-templates-unordered", "throw-placement-deleted-function-and-template"):
    skip("expressions/new-delete/" + nm, 1, "DED-39 is not modeled")
e = Env(); e.class_templates["List"] = lambda a: ClassDef()
tkl = tmpl([TP("Ts", pack=True)], [List(Expand(Ts), I)])
rejects("templates/packs/nondeduced-before-end-call", "take(List<char, int>)", [tkl], [pv(List(CH, I))], env=e, rule="DED-35")
skip("templates/specialization/partial-placeholder-pattern-const", 2, "partial specialization matching (TSP-14) is outside the component")
skip("templates/specialization/partial-constant-type-exact", 3, "partial specialization matching (TSP-14) is outside the component")
e = Env()
e.class_templates["Base"] = lambda a: ClassDef()
e.class_templates["Derived"] = lambda a: ClassDef(bases=[spec("Base", *a)], body_error=False)
bs = tmpl(["T"], [Ptr(spec("Base", T))], T)
chk("templates/instantiation/class-deduction-from-derived", "base(Derived<int>*) is int", bs, [lv(Ptr(spec("Derived", I)))], {"T": I}, env=e)
e2 = Env()
e2.class_templates["Base"] = lambda a: ClassDef()
e2.class_templates["Derived"] = lambda a: ClassDef(bases=[spec("Base", *a)], body_error=True)
r = deduce(bs, [lv(Ptr(spec("Derived", I)))], (), e2)
record("templates/instantiation/class-deduction-from-derived-error", "base(p) instantiates Derived<int>: reject", isinstance(r, HardError), str(r))
for f in ("W12/sort", "W12/compose", "W15/shapes", "W16/evaluator", "W16/textstats", "W18/container", "roadmap/r7-grid", "roadmap/r9-text"):
    skip("programs/" + f, 1, "a whole program; decision 0054 says it decides no rule alone")

# ---------------------------------------------------------------- report
right = sum(1 for r in results if r[2])
print(f"frozen sha1 {FROZEN}")
print(f"{right} of {len(results)} assertions right; {sum(n for _, n, _ in skipped)} assertions not expressible")
tests = sorted({r[0] for r in results})
print(f"{len(tests)} tests scored")
print()
for test, what, ok, why in results:
    if not ok:
        print(f"WRONG {test}: {what}\n      {why}")
print()
for test, what, ok, why in results:
    if ok and why.startswith("rule "):
        print(f"RULE  {test}: {what}: {why}")
print()
for test, what, rs in inner_rules:
    print(f"INNER {test}: {what}: {rs}")
print()
for t, n, why in skipped:
    print(f"SKIP  {t} ({n}): {why}")
