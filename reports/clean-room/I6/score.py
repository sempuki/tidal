#!/usr/bin/env python3
"""Score the frozen depnames.py against the tests that cite the DEP rules and
INST-1 to INST-13, and against the three probe grids of the dependent-names page.

Each case is one assertion. The expected value comes from the test's
tidal-expect line (or a value the test checks), or from the grid table.
An outcome is accept (run counts as accept) or reject. The component's
answer is reject, accept, or either. "either" comes from a no-diagnostic or
unspecified rule (strength "may", INST-3, INST-11) and matches both outcomes.
A split grid cell matches both outcomes."""
import hashlib, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
FROZEN = open(os.path.join(HERE, "depnames.sha1")).read().split()[0]
assert hashlib.sha1(open(os.path.join(HERE, "depnames.py"), "rb").read()).hexdigest() == FROZEN, "depnames.py changed"
sys.path.insert(0, HERE)
from depnames import *  # noqa

T, U, C = TParam("T"), TParam("U"), TParam("C")
N = TParam("N", "const", Fund("int"))
INT, LONG, DBL, VOID, CHAR = Fund("int"), Fund("long"), Fund("double"), Fund("void"), Fund("char")
Item = ClassType("Item", "lib")
Tag = ClassType("Tag", "lib")
SItem = ClassType("Item", "shop")

CASES = []


def case(src, cid, expected, got, note=""):
    CASES.append((src, cid, expected, got, note))


def ok(expected, got):
    if expected == "split" or got == "either":
        return True
    e = "accept" if expected == "run" else expected
    return e == got


def by_strength(s):
    return {"must": "reject", "may": "either", "none": "accept"}[s]


SECOND_PHASE_DECIDES = "--second-phase" in sys.argv


def kw_verdict(res, instantiated, at_inst=None):
    """Strict reading (default): every error that keywords() marks at-instantiation
    rejects an instantiated template. With --second-phase, when the case also calls
    at_instantiation() with the member that the arguments give, that call decides."""
    if any(s == "must" for _, s in res.errors):
        return "reject"
    if not instantiated:
        return "accept"
    if SECOND_PHASE_DECIDES and at_inst is not None:
        return "reject" if at_inst else "accept"
    if any(s == "at-instantiation" for _, s in res.errors):
        return "reject"
    if at_inst:
        return "reject"
    return "accept"


def bind_verdict(b):
    return "reject" if b.when == "not-found" else "accept"


def ft(params=(T,), locals=None, globals=None, pos=1, ns=""):
    return Ctx(kind="function", params=params, locals=locals or {}, globals=globals or {}, pos=pos, ns=ns,
               in_class_template=False)


def ct(name="Box", params=(T,), **kw):
    return Ctx(kind="class", name=name, params=params, **kw)


def chosen_tag(r):
    return r.chosen.tag if r.chosen else r.rule


D = "dependent-names/"
I = "instantiation/"

# ----------------------------------------------------------------- DEP-9, DEP-10, DEP-38: lookup of calls
f = ft(locals={"item": Var(T)})
call = Call(Id("probe"), (Id("item"),))
case(D + "argument-lookup-better-later", "binds at instantiation with ADL", "instantiation+adl", binding(call, f).when)
prog = [FuncDecl("probe", (Item, DBL), "lib", 0, tag="1"), FuncDecl("probe", (Item, INT), "lib", 2, tag="2")]
case(D + "argument-lookup-better-later", "chooses probe(Item, int)", "2",
     chosen_tag(resolve_call("probe", (Item, INT), f, prog, True, 3, 4)))
prog = [FuncDecl("probe", (Item,), "lib", 2, tag="7")]
case(D + "argument-lookup-later", "finds lib::probe declared later", "7",
     chosen_tag(resolve_call("probe", (Item,), f, prog, True, 3, 4)))
prog = [FuncDecl("probe", (Item, INT), "lib", 0, tag="nodefault"),
        FuncDecl("probe", (Item, INT), "lib", 2, defaults=1, tag="4")]
case(D + "argument-lookup-later-default", "takes the default declared before the point", "4",
     chosen_tag(resolve_call("probe", (Item,), f, prog, True, 3, 5)))
prog = [FuncDecl("probe", (Item, LONG), "lib", 0, tag="1")]
case(D + "argument-lookup-other-unit-ambiguous", "this unit's probe(Item, long)", "1",
     chosen_tag(resolve_call("probe", (Item, INT), f, prog, True, 2, 3)))
prog = [FuncDecl("probe", (Item, DBL), "lib", 0, tag="1")]
case(D + "argument-lookup-other-unit", "this unit's probe(Item, double)", "1",
     chosen_tag(resolve_call("probe", (Item, INT), f, prog, True, 2, 3)))

f = ft()
c = Call(Id("probe"), (Cast(Tag, None, "functional"), Cast(T, Lit(), "functional")))
case(D + "dependent-call-late", "T(0) makes the call dependent", "instantiation+adl", binding(c, f).when)
prog = [FuncDecl("probe", (Tag, DBL), "lib", 0, tag="1"), FuncDecl("probe", (Tag, INT), "lib", 2, tag="2")]
case(D + "dependent-call-late", "chooses probe(Tag, int)", "2",
     chosen_tag(resolve_call("probe", (Tag, INT), f, prog, True, 3, 4)))
f = ft(params=(TParam("T", pack=True),), locals={"values": Var(TParam("T", pack=True))})
case(D + "dependent-call-pack", "a pack expansion argument", "instantiation+adl",
     binding(Call(Id("probe"), (Expand(Id("values")),)), f).when)
case(D + "dependent-call-pack", "finds lib::probe", "7",
     chosen_tag(resolve_call("probe", (Item,), f, [FuncDecl("probe", (Item,), "lib", 2, tag="7")], True, 3, 4)))
f = ft()
case(D + "dependent-call-template-arguments", "dependent explicit arguments", "instantiation+adl",
     binding(Call(Id("probe", (T,)), (Cast(Tag, None, "functional"),)), f).when)
case(D + "dependent-call-template-arguments", "finds the template lib::probe", "t",
     chosen_tag(resolve_call("probe", (Tag,), f, [FuncDecl("probe", ("any",), "lib", 2, template=True, tag="t")],
                             True, 3, 4)))
f = ft(locals={"t": Var(T)})
case(D + "dependent-call-braced-list", "a braced list with a dependent element", "instantiation+adl",
     binding(Call(Id("helper"), (Braced((Id("t"),)),)), f).when)
case(D + "dependent-call-braced-list", "nothing checked before instantiation", "accept",
     by_strength(pre_checks(Ctx(kind="function", params=(T,), locals={"t": Var(T)}, in_class_template=False,
                                body=[Construct("expr", Call(Id("helper"), (Braced((Id("t"),)),)), valid="some")]))[0][2]))

PS = TParam("T", pack=True)
f = ft(params=(PS,))
ptype = Ptr(Func(VOID, (Item, PS), pack=True))
case(D + "dependent-type-function-pack", "a function type with a pack is dependent", True, dependent_type(ptype, f))
f2 = ft(locals={"p": Var(ptype)}, params=(PS,))
case(D + "dependent-type-function-pack", "price(p) binds with ADL", "instantiation+adl",
     binding(Call(Id("price"), (Id("p"),)), f2).when)
case(D + "dependent-type-function-pack", "ADL through the parameter type finds shop::price", "7",
     chosen_tag(resolve_call("price", (Ptr(Func(VOID, (Item,))),), f2,
                             [FuncDecl("price", (Ptr(Func(VOID, (Item,))),), "lib", 2, tag="7")], True, 3, 4)))
B = TParam("B", "const", Fund("bool"))
f = ft(params=(B,))
case(D + "dependent-type-noexcept", "a value-dependent noexcept makes the type dependent", True,
     dependent_type(Ptr(Func(VOID, (SItem,), noexcept=Id("B"))), f))
f = ft(params=(N,))
case(D + "dependent-type-template-id", "Tag<N> is dependent", True,
     dependent_type(Spec(TemplateRef("Tag", "shop"), (Id("N"),)), f))
f = ft(params=(PS,))
case(D + "dependent-type-template-id", "List<T...> is dependent", True,
     dependent_type(Spec(TemplateRef("List", "shop"), (Expand(PS),)), f))
f = ft(params=(TParam("auto_item"),), locals={"item": Var(TParam("auto_item"))})
case(D + "generic-lambda-argument-lookup", "probe(item) binds with ADL", "instantiation+adl",
     binding(Call(Id("probe"), (Id("item"),)), f).when)
case(D + "generic-lambda-argument-lookup", "finds lib::probe declared after the lambda", "4",
     chosen_tag(resolve_call("probe", (Item,), f, [FuncDecl("probe", (Item,), "lib", 2, tag="4")], True, 3, 5)))

f = ft(locals={"a": Var(T), "b": Var(T)})
case(D + "operator-lookup-later", "a + b binds with ADL", "instantiation+adl",
     binding(Op("+", (Id("a"), Id("b"))), f).when)
case(D + "operator-lookup-later", "finds shop::operator+", "7",
     chosen_tag(resolve_call("operator+", (SItem, SItem), f,
                             [FuncDecl("operator+", (SItem, SItem), "shop", 2, tag="7")], True, 3, 4)))
case(D + "operator-ordinary-lookup-later", "does not find ::operator+ declared later", "reject",
     "reject" if resolve_call("operator+", (SItem, INT), f,
                              [FuncDecl("operator+", (SItem, INT), "", 2, tag="7")], True, 3, 4).chosen is None
     else "accept")
f = ft(locals={"item": Var(T)})
case(D + "ordinary-lookup-later-class", "does not find ::probe declared later", "reject",
     "reject" if resolve_call("probe", (Item,), f, [FuncDecl("probe", (Item,), "", 2)], True, 3, 4).chosen is None
     else "accept")
case(D + "ordinary-lookup-later-fundamental", "an int argument adds no namespace", "reject",
     "reject" if resolve_call("probe", (INT,), f, [FuncDecl("probe", (INT,), "", 2)], True, 3, 4).chosen is None
     else "accept")

# ----------------------------------------------------------------- DEP-5, DEP-6, DEP-3
f = ft(globals={"pick": "function"})
case(D + "independent-bound-early", "pick(1) binds at the definition", "definition",
     binding(Call(Id("pick"), (Lit(),)), f).when)
case(D + "independent-bound-early", "chooses pick(double)", "1",
     chosen_tag(resolve_call("pick", (INT,), f, [FuncDecl("pick", (DBL,), "", 0, tag="1"),
                                                 FuncDecl("pick", (INT,), "", 2, tag="2")], False, 3, 4)))
f = ft(globals={"probe": "function"}, locals={"v": Var(T)})
case(D + "independent-call-early", "sizeof(T) is not type-dependent", False, type_dependent(Sizeof(T), f))
case(D + "independent-call-early", "probe(Tag{}, sizeof(T)) binds at the definition", "definition",
     binding(Call(Id("probe"), (Cast(Tag, None, "functional"), Sizeof(T))), f).when)
case(D + "independent-call-early", "probe(Tag{}, static_cast<int>(v)) binds at the definition", "definition",
     binding(Call(Id("probe"), (Cast(Tag, None, "functional"), Cast(INT, Id("v")))), f).when)
case(D + "independent-call-early", "chooses probe(Tag, double)", "1",
     chosen_tag(resolve_call("probe", (Tag, LONG), f, [FuncDecl("probe", (Tag, DBL), "lib", 0, tag="1"),
                                                       FuncDecl("probe", (Tag, INT), "lib", 2, tag="3")], False, 3, 4)))
f = ft(params=(PS,), globals={"probe": "function"})
case(D + "independent-call-pack-size", "probe(Tag{}, sizeof...(T)) binds at the definition", "definition",
     binding(Call(Id("probe"), (Cast(Tag, None, "functional"), SizeofPack("T"))), f).when)
f = ft()
case(D + "independent-not-declared", "helper(1) is not found at the definition", "reject",
     bind_verdict(binding(Call(Id("helper"), (Lit(),)), f)))
case(D + "independent-error-unused-name", "helper(1), never instantiated", "reject",
     by_strength(pre_checks(Ctx(kind="function", params=(T,), in_class_template=False,
                                body=[Construct("name", Call(Id("helper"), (Lit(),)))]))[0][2]))
case(D + "independent-type-not-declared", "Later is not found at the definition", "reject",
     "reject" if lookup_unqualified("Later", f).where == "none" else "accept")
case(D + "independent-error-unused-call", "take(1), never instantiated", "reject",
     by_strength(pre_checks(Ctx(kind="function", params=(T,), in_class_template=False, globals={"take": 1},
                                body=[Construct("expr", Call(Id("take"), (Lit(),)), valid="none")]))[0][2]))
case(D + "independent-error-unused-conversion", "int* p = 1, never instantiated", "reject",
     by_strength(pre_checks(Ctx(kind="function", params=(T,), in_class_template=False,
                                body=[Construct("expr", Lit(), valid="none")]))[0][2]))

# member alias of the current instantiation (DEP-1)
cb = ct(aliases={"item": ClassType("Item", "shop")}, members={"item": MemberDecl("alias", ClassType("Item", "shop"))},
        locals={"i": Var(AliasName("item"))}, globals={"pick": 1})
case(D + "member-alias-independent", "item is not a dependent type", False, dependent_type(AliasName("item"), cb))
case(D + "member-alias-independent", "pick(i) binds at the definition", "definition",
     binding(Call(Id("pick"), (Id("i"),)), cb).when)
case(D + "member-alias-independent", "chooses pick(Item, long = 0)", "1",
     chosen_tag(resolve_call("pick", (ClassType("Item", "shop"),), cb,
                             [FuncDecl("pick", (ClassType("Item", "shop"), LONG), "shop", 0, defaults=1, tag="1"),
                              FuncDecl("pick", (ClassType("Item", "shop"),), "shop", 2, tag="2")], False, 3, 4)))

# ----------------------------------------------------------------- DEP-2, DEP-4
cb = ct(members={"make": MemberDecl("function", deduced_return=True)})
case(D + "type-dependent-deduced-return", "make() is type-dependent", True, type_dependent(Call(Id("make")), cb))
case(D + "type-dependent-deduced-return", "price(make()) binds with ADL", "instantiation+adl",
     binding(Call(Id("price"), (Call(Id("make")),)), cb).when)
f = ft(params=(PS,), locals={"t": Var(PS)})
case(D + "type-dependent-fold", "a fold is type-dependent", True, type_dependent(Fold(",", Id("t")), f))
f = ft(params=(TParam("V", "const", Placeholder()),))
case(D + "type-dependent-placeholder-parameter", "V of type auto is type-dependent", True,
     type_dependent(Id("V"), f))
f = ft(locals={"t": Var(T), "copy": Var(Placeholder(), init=Id("t"))})
case(D + "type-dependent-placeholder-variable", "auto copy = t is type-dependent", True,
     type_dependent(Id("copy"), f))
f = ft(locals={"t": Var(T), "a": Var(None, binding_of=T)})
case(D + "type-dependent-set-and-binding", "make<T> is type-dependent", True,
     type_dependent(Call(Id("make", (T,)), (Lit(),)), f))
case(D + "type-dependent-set-and-binding", "T::make(1) is type-dependent", True,
     type_dependent(Call(QualId(T, "make"), (Lit(),)), f))
case(D + "type-dependent-set-and-binding", "a structured binding of a T is type-dependent", True,
     type_dependent(Id("a"), f))
case(D + "type-dependent-set-and-binding", ".size() waits for the instantiation", "instantiation",
     binding(Member(Call(Id("make", (T,)), (Lit(),)), "size"), f).when)
f = ft(params=(N,))
bound = Op("-", (Id("N"), Lit()))
case(D + "value-dependent-bound", "N - 1 is value-dependent", True, value_dependent(bound, f))
case(D + "value-dependent-bound", "int[N - 1] is a dependent type", True, dependent_type(Arr(INT, bound), f))
case(D + "value-dependent-bound-fails", "the bound is checked for the specialization", "accept",
     by_strength(pre_checks(Ctx(kind="function", params=(N,), in_class_template=False,
                                body=[Construct("decl", Arr(INT, bound), valid="some")]))[0][2]))
f = ft(params=(N,), locals={"count": Var(INT, const=True, init=Id("N"))})
case(D + "value-dependent-constant-variable", "const int count = N is value-dependent", True,
     value_dependent(Id("count"), f))
f = ft(params=(PS,))
case(D + "value-dependent-pack", "sizeof...(T) + 1 is value-dependent", True,
     value_dependent(Op("+", (SizeofPack("T"), Lit())), f))
NP = TParam("N", "const", INT, pack=True)
case(D + "value-dependent-pack", "(N + ...) is value-dependent", True,
     value_dependent(Fold("+", Id("N")), ft(params=(NP,))))

# checked-at-instantiation (DEP-1, DEP-28)
cs = ct("Shelf", (T, N), aliases={"row": Arr(T, Id("N"))})
case(D + "checked-at-instantiation", "T[N] is dependent", True, dependent_type(Arr(T, Id("N")), cs))
case(D + "checked-at-instantiation", "T* (*)(T&) is dependent", True, dependent_type(Ptr(Func(Ptr(T), (Ref(T),))), cs))
r = keywords(Qual(T, "size_type", kw_typename=True), cs, "member-type")
case(D + "checked-at-instantiation", "typename T::size_type", "run",
     kw_verdict(r, True, at_instantiation(Qual(T, "size_type", kw_typename=True), "type", r.reads_as)))
case(D + "checked-at-instantiation", "T::limit read as a value", "run",
     "reject" if at_instantiation(QualId(T, "limit"), "value", "value") else "accept")
case(D + "dependent-error-at-instantiation", "T::count with T = int", "reject",
     "reject" if at_instantiation(QualId(T, "count"), "none", "value") else "accept")
case(D + "dependent-error-unused", "p->size() + T::count, never instantiated", "accept",
     by_strength(pre_checks(Ctx(kind="function", params=(T,), in_class_template=False, locals={"p": Var(Ptr(T))},
                                body=[Construct("expr", Op("+", (Call(Member(Id("p"), "size", True)), QualId(T, "count"))),
                                                valid="some")]))[0][2]))
case(D + "per-specialization", "T::value binds at each instantiation", "instantiation",
     binding(QualId(T, "value"), ft()).when)
case(D + "per-specialization-type", "typename T::type binds at each instantiation", "instantiation",
     keywords(Qual(T, "type", kw_typename=True), ft(), "sizeof").binds)

# ----------------------------------------------------------------- DEP-11, DEP-12, DEP-14, DEP-36
BT = Spec(TemplateRef("Base"), (T,))
cd = ct("Derived", bases=(BT,), globals={"count": "variable"})
case(D + "dependent-base-skipped", "count finds ::count", "global", lookup_unqualified("count", cd).where)
cd2 = ct("Derived", bases=(BT,))
case(D + "dependent-base-not-found", "count only in Base<T>", "reject",
     bind_verdict(binding(Id("count"), cd2, in_dependent_base=True)))
case(D + "independent-error-unused-base-member", "count only in Base<T>, never instantiated", "reject",
     by_strength(pre_checks(ct("Derived", bases=(BT,), body=[Construct("name", Id("count"), dependent=True)]))[0][2]))
case(D + "dependent-base-type-not-found", "size only in Base<T>", "reject",
     "reject" if lookup_unqualified("size", cd2, True).where == "none" else "accept")
case(D + "dependent-base-qualified", "this->count waits", "instantiation", binding(Member(This(), "count", True), cd).when)
case(D + "dependent-base-qualified", "Derived::count waits", "instantiation", binding(QualId(Injected(), "count"), cd).when)
case(D + "dependent-base-qualified", "Base<T>::count waits", "instantiation", binding(QualId(BT, "count"), cd).when)
case(D + "dependent-base-conversion-function", "operator T waits", "instantiation",
     binding(Id("operator T", (T,)), cd2, in_dependent_base=True).when)
case(D + "dependent-base-conversion-function-independent", "operator int is not found", "reject",
     bind_verdict(binding(Id("operator int", (INT,)), cd2, in_dependent_base=True)))
Fixed = ClassType("Fixed")
ci = ct("Derived", bases=(ClassType("Base"),), base_members={ClassType("Base"): {"count": MemberDecl("value", INT)}},
        globals={"count": 1})
case(D + "independent-base-searched", "count finds Base::count", "base-member", lookup_unqualified("count", ci).where)
cbth = ct("Both", bases=(Fixed, T), base_members={Fixed: {"m": MemberDecl("value", INT),
                                                          "f": MemberDecl("function", INT)}})
case(D + "unqualified-member-kept", "m keeps Fixed::m", "definition", binding(Id("m"), cbth).when)
case(D + "unqualified-member-kept", "m finds a member", "base-member", lookup_unqualified("m", cbth).where)
case(D + "unqualified-member-function-kept", "f() keeps Fixed::f", "definition", binding(Call(Id("f")), cbth).when)
BaseN = ClassType("Base")
ch = ct("Inside", params=(N,), bases=(BaseN,), base_members={BaseN: {"N": MemberDecl("value", INT)}})
case(D + "base-member-hides-constant-parameter", "N finds Base::N", "base-member", lookup_unqualified("N", ch).where)
case(D + "base-member-hides-constant-parameter", "N in Outside<N>::get finds Base::N", "base-member",
     lookup_unqualified("N", ct("Outside", params=(N,), bases=(BaseN,),
                                base_members={BaseN: {"N": MemberDecl("value", INT)}})).where)
ch = ct("Inside", params=(U,), bases=(BaseN,), base_members={BaseN: {"U": MemberDecl("alias", INT)}})
case(D + "base-member-hides-parameter", "U finds Base::U", "base-member", lookup_unqualified("U", ch).where)
case(D + "base-member-hides-parameter", "U in Outside<U>::get finds Base::U", "base-member",
     lookup_unqualified("U", ct("Outside", params=(U,), bases=(BaseN,),
                                base_members={BaseN: {"U": MemberDecl("alias", INT)}})).where)
OT = Spec(TemplateRef("Outer"), (T,))
cin = ct("Outer", nested="Inner", bases=(OT,), base_members={OT: {"value": MemberDecl("function", INT),
                                                                   "value_type": MemberDecl("alias", INT)}},
         globals={"value": 1})
case(D + "current-instantiation-enclosing-base", "Outer<T> is not a dependent base here", False,
     has_dependent_base(cin))
case(D + "current-instantiation-enclosing-base", "value() finds Outer<T>::value", "base-member",
     lookup_unqualified("value", cin).where)

# ----------------------------------------------------------------- DEP-13, DEP-35
case(D + "qualified-member-found-again", "Both::m finds two at the instantiation", "reject",
     by_strength(second_lookup("Fixed::m", ["Fixed::m", "Extra::m"])[1]))
case(D + "this-member-ambiguous", "this->m finds two at the instantiation", "reject",
     by_strength(second_lookup("Fixed::m", ["Fixed::m", "Extra::m"])[1]))
case(D + "qualified-member-function-other", "Both::f finds Extra::f", "accept",
     by_strength(second_lookup("Fixed::f", ["Extra::f"])[1]))
case(D + "qualified-member-other-at-instantiation", "this->m and Both::m find Extra::m", "accept",
     by_strength(second_lookup("Fixed::m", ["Extra::m"])[1]))

# ----------------------------------------------------------------- DEP-15 to DEP-18, DEP-39
BX = TemplateRef("Box")
cbx = ct(members={"type": MemberDecl("alias", INT)}, alias_templates={"Same": AliasTemplate(("U",), Spec(BX, (TParam("U"),)))})
r = keywords(Qual(AliasSpec("Same", (T,)), "type"), cbx, "local", declarator_name="a")
case(D + "current-instantiation-alias", "Same<T>::type needs no typename", "run", kw_verdict(r, True))
for q, lab in ((Injected(), "Box::type"), (Spec(BX, (T,)), "Box<T>::type"), (Spec(BX, (T,)), "::Box<T>::type")):
    case(D + "current-instantiation-forms", lab, "run", kw_verdict(keywords(Qual(q, "type"), cbx, "local",
                                                                            declarator_name="a"), True))
cnest = ct("Outer", nested="Inner", members={"type": MemberDecl("alias", INT)})
case(D + "current-instantiation-nested", "Inner::type", "run",
     kw_verdict(keywords(Qual(Nested("Inner"), "type"), cnest, "local", declarator_name="a"), True))
case(D + "current-instantiation-nested", "Outer<T>::Inner::type", "run",
     kw_verdict(keywords(Qual(Qual(Spec(TemplateRef("Outer"), (T,)), "Inner"), "type"), cnest, "local",
                         declarator_name="b"), True))
cpa = ct(aliases={"self": T}, members={"type": MemberDecl("alias", INT)})
case(D + "current-instantiation-parameter-alias", "Box<self>::type", "run",
     kw_verdict(keywords(Qual(Spec(BX, (AliasName("self"),)), "type"), cpa, "local", declarator_name="a"), True))
cpp = ct(partial_args=(Ptr(T),), aliases={"pointer": Ptr(T)}, members={"type": MemberDecl("alias", INT)})
case(D + "current-instantiation-partial-alias", "Box<pointer>::type", "run",
     kw_verdict(keywords(Qual(Spec(BX, (AliasName("pointer"),)), "type"), cpp, "local", declarator_name="a"), True))
case(D + "current-instantiation-partial", "Box<T*>::type", "run",
     kw_verdict(keywords(Qual(Spec(BX, (Ptr(T),)), "type"), cpp, "local", declarator_name="a"), True))
case(D + "current-instantiation-partial", "Box::type", "run",
     kw_verdict(keywords(Qual(Injected(), "type"), cpp, "local", declarator_name="b"), True))
case(D + "partial-primary-form-needs-typename", "Box<T>::type inside Box<T*>", "reject",
     kw_verdict(keywords(Qual(Spec(BX, (T,)), "type"), ct(partial_args=(Ptr(T),)), "local", declarator_name="a"), False))
case(D + "other-specialization-needs-typename", "Box<T*>::type inside Box<T>", "reject",
     kw_verdict(keywords(Qual(Spec(BX, (Ptr(T),)), "type"), cbx, "local", declarator_name="a"), False))
cm = ct(params=(N,), members={"M": MemberDecl("value", INT, static_const_of="N"), "type": MemberDecl("alias", INT)})
case(D + "current-instantiation-constant-member", "Box<M>::type is another specialization", "reject",
     kw_verdict(keywords(Qual(Spec(BX, (Id("M"),)), "type"), cm, "local", declarator_name="local"), True))
cg = ct("Gauge", params=(N,), locals={"o": Var(Ref(CV(Spec(TemplateRef("Gauge"), (Lit(),)))))})
case(D + "member-of-other-specialization-constant-argument", "Gauge<0> is not dependent", False,
     dependent_type(Spec(TemplateRef("Gauge"), (Lit(),)), cg))
case(D + "member-of-other-specialization-constant-argument", "o.level binds at the definition", "definition",
     binding(Member(Id("o"), "level"), cg).when)
cmt = ct("Meter", params=(N,), locals={"o": Var(Ref(CV(Spec(TemplateRef("Meter"), (Lit(),)))))},
         body=[Construct("expr", Member(Id("o"), "missing"), valid="none")])
case(D + "other-specialization-constant-argument-member-missing", "o.missing in Meter<0>", "reject",
     by_strength(pre_checks(cmt)[0][2]))
case(D + "other-specialization-constant-argument-no-typename", "Meter<0>::unit", "run",
     kw_verdict(keywords(Qual(Spec(TemplateRef("Meter"), (Lit(),)), "unit"), cmt, "local", declarator_name="zero"), True))
cdb = ct(bases=(BT,))
case(D + "current-member-dependent-base", "Box::missing waits", "instantiation", binding(QualId(Injected(), "missing"), cdb).when)
case(D + "current-member-dependent-base-function", "Box::make() waits", "instantiation",
     binding(Call(QualId(Injected(), "make")), cdb).when)
case(D + "current-member-missing", "Box::missing, never instantiated", "reject",
     by_strength(pre_checks(ct(body=[Construct("name", QualId(Injected(), "missing"))]))[0][2]))
case(D + "current-member-missing-used", "Box::missing, instantiated", "reject",
     bind_verdict(binding(QualId(Injected(), "missing"), ct())))
case(D + "this-member-dependent-base", "this->missing() waits", "instantiation",
     binding(Call(Member(This(), "missing", True)), ct("Gauge", bases=(BT,))).when)
for t in ("this-member-missing", "this-member-missing-specialized"):
    case(D + t, "this->missing, never instantiated", "reject",
         by_strength(pre_checks(ct("Gauge", body=[Construct("name", Member(This(), "missing", True))]))[0][2]))
case(D + "this-member-missing-used", "this->missing(), instantiated", "reject",
     bind_verdict(binding(Member(This(), "missing", True), ct("Gauge"))))
f = ft(locals={"t": Var(T)})
case(D + "member-access-dependent-object-late", "t.later() waits", "instantiation", binding(Member(Id("t"), "later"), f).when)
case(D + "member-access-dependent-object-never-found", "t.nothing(), never instantiated", "accept",
     by_strength(pre_checks(Ctx(kind="function", params=(T,), in_class_template=False, locals={"t": Var(T)},
                                body=[Construct("expr", Call(Member(Id("t"), "nothing")), valid="some")]))[0][2]))

# ----------------------------------------------------------------- DEP-24 with the current instantiation
cw = ct("Wrap", bases=(BT,), locals={"w": Var(Ref(CV(Injected())))})
r = keywords(Member(Id("w"), "from_base", targs=(INT,)), cw)
case(D + "member-template-current-instantiation-dependent-base-needs-template", "w.from_base<int>()", "reject",
     kw_verdict(r, True))
cw2 = ct("Wrap", members={"get": MemberDecl("template")}, locals={"p": Var(Ptr(CV(Injected()))), "l": Var(Injected()),
                                                                   "w": Var(Ref(CV(Injected())))})
case(D + "member-template-current-instantiation-pointer", "p->get<int>()", "run",
     kw_verdict(keywords(Member(Id("p"), "get", True, targs=(INT,)), cw2), True))
case(D + "member-template-current-instantiation-pointer", "l.get<int>()", "run",
     kw_verdict(keywords(Member(Id("l"), "get", targs=(INT,)), cw2), True))
case(D + "member-template-through-current-instantiation-object", "w.get<int>()", "run",
     kw_verdict(keywords(Member(Id("w"), "get", targs=(INT,)), cw2), True))

# ----------------------------------------------------------------- DEP-19 to DEP-27, DEP-34
f = ft(params=(C,), locals={"object": Var(T), "address": Var(Ptr(T))})
ft_ = ft(locals={"object": Var(T), "address": Var(Ptr(T)), "t": Var(T)})
q = Qual(T, "box", True, True, (INT,))
case(D + "template-keyword-forms", "typename T::template box<int>", "run",
     kw_verdict(keywords(q, ft_, "local", declarator_name="b"), True, at_instantiation(q, "class-template", "type")))
q = Qual(T, "pointer", True, True, (INT,))
case(D + "template-keyword-forms", "typename T::template pointer<int>", "run",
     kw_verdict(keywords(q, ft_, "local", declarator_name="p"), True, at_instantiation(q, "alias-template", "type")))
q = Qual(T, "make", False, True, (CHAR,))
r = keywords(q, ft_, "other", intended="value")
case(D + "template-keyword-forms", "T::template make<char>()", "run",
     kw_verdict(r, True, at_instantiation(q, "function-template", r.reads_as)))
case(D + "template-keyword-forms", "object.template get<int>()", "run",
     kw_verdict(keywords(Member(Id("object"), "get", kw_template=True, targs=(INT,)), ft_), True))
case(D + "template-keyword-forms", "address->template get<long>()", "run",
     kw_verdict(keywords(Member(Id("address"), "get", True, kw_template=True, targs=(LONG,)), ft_), True))
q = Qual(T, "box", False, True)
r = keywords(q, ft_, "template-argument", intended="template-arg")
case(D + "template-keyword-forms", "Holder<T::template box>", "run", kw_verdict(r, True, at_instantiation(q, "class-template", "template-id")))
case(D + "template-keyword-forms", "Has::template make<char>() outside a template", "run",
     kw_verdict(keywords(Qual(ClassType("Has"), "make", False, True, (CHAR,)), Ctx(in_class_template=False), intended="value"), True))
for t in ("template-keyword-destructor", "template-keyword-destructor-dot"):
    case(D + t, "template before ~T", "reject",
         kw_verdict(keywords(Member(Id("p"), "~T", True, kw_template=True), ft(locals={"p": Var(Ptr(T))}),
                             destructor=True), True))
for t in ("template-keyword-in-declarator-class", "template-keyword-in-declarator", "template-keyword-in-declarator-static"):
    case(D + t, "Box<T>::template name in a declarator", "reject",
         kw_verdict(keywords(Qual(Spec(BX, (T,)), "x", kw_template=True), ct(), declares=True), False))
q = Qual(T, "box", True, False, (INT,))
case(D + "template-keyword-needed-after-typename", "typename T::box<int>", "reject",
     kw_verdict(keywords(q, ft_, "local", declarator_name="b"), True))
case(D + "template-keyword-needed-alias-target", "using alias = T::box<int>", "reject",
     kw_verdict(keywords(Qual(T, "box", targs=(INT,)), ct("Use"), "alias"), True))
case(D + "template-keyword-needed-base", ": T::box<int>", "reject",
     kw_verdict(keywords(Qual(T, "box", targs=(INT,)), ct("Use"), "base"), True))
case(D + "template-keyword-needed-member", "object->get<int>()", "reject",
     kw_verdict(keywords(Member(Id("object"), "get", True, targs=(INT,)), ft(locals={"object": Var(Ptr(T))})), True))
q = Qual(T, "make", targs=(INT,))
r = keywords(q, ft_, "other", intended="value")
case(D + "template-keyword-needed-qualified", "T::make<int>()", "reject",
     kw_verdict(r, True, at_instantiation(q, "function-template", r.reads_as)))
q = Qual(T, "box")
r = keywords(q, ct("Uses"), "template-argument", intended="template-arg")
case(D + "template-keyword-needed-template-argument", "Holder<T::box>", "reject", kw_verdict(r, True))
q = Qual(T, "box", True, True)
r = keywords(q, ft_, "local", intended="type", declarator_name="b")
case(D + "template-keyword-no-list-class", "typename T::template box b(1)", "accept",
     kw_verdict(r, True, at_instantiation(q, "class-template", r.reads_as)))
m = Member(Id("object"), "field", kw_template=True)
r = keywords(m, ft_)
case(D + "template-keyword-no-list", "object.template field", "reject", kw_verdict(r, True, at_instantiation(m, "value", "value")))
q = Qual(T, "field", False, True)
r = keywords(q, ft_, intended="value")
case(D + "template-keyword-no-list-static", "T::template field", "reject", kw_verdict(r, True, at_instantiation(q, "value", "value")))
q = Qual(T, "plain", False, True, (INT,))
r = keywords(q, ft_, intended="value")
case(D + "template-keyword-not-template", "T::template plain<int>()", "reject",
     kw_verdict(r, True, at_instantiation(q, "value", r.reads_as)))
case(D + "template-keyword-not-template-unused", "T::template plain<int>(), never instantiated", "accept", kw_verdict(r, False))
q = Qual(T, "box", False, True)
r = keywords(q, ct("Outer"), "default-type-arg", intended="template-arg")
case(D + "template-keyword-template-template-default", "= T::template box", "accept",
     kw_verdict(r, True, at_instantiation(q, "class-template", "template-id")))

q = Qual(T, "name", kw_typename=True)
for t, mk in (("typename-finds-data-member", "value"), ("typename-finds-nothing", "none"), ("typename-finds-value", "value")):
    case(D + t, "typename T::name finds " + mk, "reject",
         kw_verdict(keywords(q, ft(), "local", declarator_name="n"), True, at_instantiation(q, mk, "type")))
case(D + "typename-needed-block", "T::type value = 0", "reject",
     kw_verdict(keywords(Qual(T, "type"), ft(), "local", declarator_name="value"), True))
q = Qual(T, "type")
r = keywords(q, ft(), "sizeof")
case(D + "typename-needed-expression", "sizeof(T::type)", "reject", kw_verdict(r, True, at_instantiation(q, "type", r.reads_as)))
case(D + "typename-needed-parameter", "int take(T::type) { ... }", "reject",
     kw_verdict(keywords(q, ft(), "ns-function-param-with-body"), False))
r = keywords(q, ft(), "template-argument")
case(D + "typename-needed-template-argument", "Box<T::type>", "reject", kw_verdict(r, True, at_instantiation(q, "type", r.reads_as)))
u = using_declaration(False, "type")
case(D + "typename-needed-using", "using Base<T>::type; type value", "reject", "reject" if isinstance(u, tuple) else "accept")
case(D + "using-typename", "using typename Base<T>::type", "run", "accept" if using_declaration(True, "type") == "type" else "reject")
case(D + "using-typename-return", "using typename Base<T>::size_type", "accept",
     "accept" if using_declaration(True, "type") == "type" else "reject")
case(D + "typename-needs-qualified-name", "typename T value", "reject", by_strength(typename_unqualified()[1]))
r = keywords(Qual(T, "type"), ft(), "ns-function-param", intended="type")
case(D + "parameter-position-variable-template", "int take(T::type); reads a value", "accept",
     kw_verdict(r, True, at_instantiation(Qual(T, "type"), "value", r.reads_as)))
case(D + "parameter-position-variable-template", "T::type is read as a value", "value", r.reads_as)
q = Qual(T, "count")
r = keywords(q, ft(), "local", intended="value")
case(D + "value-read-as-value", "int n(T::count)", "run", kw_verdict(r, True, at_instantiation(q, "value", r.reads_as)))
r = keywords(q, ft(), "other", intended="value")
case(D + "value-was-type", "return T::count with count a type", "reject", kw_verdict(r, True, at_instantiation(q, "type", r.reads_as)))
HasT = ClassType("Has")
case(D + "typename-not-needed", "typename Has::type outside a template", "run",
     kw_verdict(keywords(Qual(HasT, "type", kw_typename=True), Ctx(in_class_template=False), "ns-variable"), True))
case(D + "typename-not-needed", "typename Has::type in a template", "run",
     kw_verdict(keywords(Qual(HasT, "type", kw_typename=True), ft(), "local", declarator_name="a"), True))
case(D + "typename-not-needed", "typename Box::type in Box", "run",
     kw_verdict(keywords(Qual(Injected(), "type", kw_typename=True), cbx, "member-type"), True))

POS_ONLY = [("Box: T::type value", "member-type"), ("Uses : T::base", "base"), ("using alias = T::type", "alias"),
            ("T::type member", "member-type"), ("T::type get()", "member-type"), ("set(T::type)", "member-param"),
            ("T::type later();", "member-type"), ("store(T::type)", "member-param"), ("friend touch(Uses, T::type)", "friend-param"),
            ("T::type Uses<T>::later()", "ns-return"), ("Uses<T>::store(T::type)", "qualified-function-param"),
            ("T::type make()", "ns-return"), ("-> T::type", "trailing-return"), ("class U = T::type", "default-type-arg"),
            ("T::type N = 7", "const-param-type"), ("T::type zero", "ns-variable"), ("new T::type", "new"),
            ("static_cast<T::type>", "named-cast"), ("[](T::type x)", "lambda-param")]
for lab, pos in POS_ONLY:
    case(D + "type-only-positions", lab, "run", kw_verdict(keywords(Qual(T, "type"), ct("Uses"), pos), True))
for lab, pos in (("Alias = T::type", "alias"), ("static_cast<T::type>", "named-cast"), ("class U = T::type", "default-type-arg"),
                 ("T::type N", "const-param-type"), ("[](T::type x)", "lambda-param")):
    case(D + "type-only-positions-more", lab, "accept", kw_verdict(keywords(Qual(T, "type"), ft(), pos), True))

# ----------------------------------------------------------------- DEP-28 to DEP-33, DEP-37, DEP-31
def one(c, ctx=None):
    ctx = ctx or Ctx(kind="function", params=(T,), in_class_template=False)
    ctx.body = [c]
    return by_strength(pre_checks(ctx)[0][2])


case(D + "base-pack-expanded-twice", "T..., T...", "reject", one(Construct("base-pack-twice")))
case(D + "binary-operator-pack-never-fits", "operator<(Item, Item, T...)", "reject",
     one(Construct("operator-pack", op="<", fixed_params=2)))
case(D + "operator-pack-never-fits", "operator~(Item, T...)", "reject", one(Construct("operator-pack", op="~", fixed_params=1)))
case(D + "constant-function-defined-later", "limit() defined after the template", "run",
     one(Construct("expr", Call(Id("limit")), later_change=True)))
case(D + "incomplete-then-complete", "sizeof(Later), Later completed later", "reject",
     one(Construct("expr", Sizeof(ClassType("Later")), later_change=True)))
case(D + "qualified-call-later-default", "lib::probe(item) with a later default", "run",
     one(Construct("expr", Call(QualId(ClassType("lib"), "probe"), (Id("item"),)), dependent=False, later_change=True)))
case(D + "no-valid-discarded-statement", "the discarded branch is never valid", "accept",
     one(Construct("discarded", valid="none")))
case(D + "no-valid-specialization", "int items[sizeof(T) - sizeof(T) - 1]", "accept",
     one(Construct("decl", Arr(INT, Op("-", (Op("-", (Sizeof(T), Sizeof(T))), Lit()))), valid="none")))
case(D + "pack-always-empty", "sizeof(void(T)) for each T", "run", one(Construct("pack-only-empty")))
case(D + "pack-always-empty-function-type", "sizeof(int(T)) for each T", "accept", one(Construct("pack-only-empty")))
case(D + "syntax-error-unused", "int value = ;", "reject", one(Construct("syntax")))

# ----------------------------------------------------------------- grids
GRID = "dependent-names/grids/"
TP_ROWS = [("alias target", "alias", "accept"), ("data member", "member-type", "accept"),
           ("member function return, in class", "member-type", "accept"),
           ("member function parameter, in class", "member-param", "accept"),
           ("member function return, outside class", "ns-return", "accept"),
           ("member function parameter, outside class", "qualified-function-param", "accept"),
           ("function template return", "ns-return", "accept"),
           ("function template parameter", "ns-function-param-with-body", "reject"),
           ("trailing return type", "trailing-return", "accept"), ("default type argument", "default-type-arg", "accept"),
           ("constant parameter type", "const-param-type", "accept"), ("static_cast type", "named-cast", "accept"),
           ("new type", "new", "accept"), ("variable in a block", "local", "reject"),
           ("sizeof operand", "sizeof", "split"), ("functional cast", "functional-cast", "split"),
           ("template argument", "template-argument", "reject"), ("lambda parameter", "lambda-param", "accept"),
           ("function declared in a block", "block-function-param", "split"),
           ("variable template type", "ns-variable", "accept"), ("friend function parameter", "friend-param", "accept")]
for row, pos, exp in TP_ROWS:
    q = Qual(T, "type")
    dn = "v" if row == "variable in a block" else None
    r = keywords(q, ft(), pos, declarator_name=dn)
    case(GRID + "typename-positions", row + " / T::type", exp, kw_verdict(r, True, at_instantiation(q, "type", r.reads_as)))
    q = Qual(T, "type", kw_typename=True)
    r = keywords(q, ft(), pos, declarator_name=dn)
    case(GRID + "typename-positions", row + " / typename T::type", "accept",
         kw_verdict(r, True, at_instantiation(q, "type", r.reads_as)))

ft_ = ft(locals={"t": Var(T), "tp": Var(Ptr(T))})
nocls = Ctx(in_class_template=False, locals={"h": Var(ClassType("Has"))})
for kwd in (False, True):
    col = "template" if kwd else "no keyword"
    exp = (lambda a, b: b if kwd else a)
    q = Qual(T, "make", False, kwd, (INT,))
    r = keywords(q, ft_, intended="value")
    case(GRID + "template-keyword", "after T:: in a call / " + col, exp("reject", "accept"),
         kw_verdict(r, True, at_instantiation(q, "function-template", r.reads_as)))
    case(GRID + "template-keyword", "after a dot / " + col, exp("reject", "accept"),
         kw_verdict(keywords(Member(Id("t"), "get", False, kwd, (INT,)), ft_), True))
    case(GRID + "template-keyword", "after an arrow / " + col, exp("reject", "accept"),
         kw_verdict(keywords(Member(Id("tp"), "get", True, kwd, (INT,)), ft_), True))
    q = Qual(T, "box", True, kwd, (INT,))
    r = keywords(q, ft_, "local", declarator_name="b")
    case(GRID + "template-keyword", "after typename T:: / " + col, exp("split", "accept"),
         kw_verdict(r, True, at_instantiation(q, "class-template", r.reads_as)))
    q = Qual(T, "ptr", True, kwd, (INT,))
    r = keywords(q, ct("P"), "alias")
    case(GRID + "template-keyword", "alias target / " + col, exp("split", "accept"),
         kw_verdict(r, True, at_instantiation(q, "alias-template", r.reads_as)))
    q = Qual(T, "box", False, kwd)
    r = keywords(q, ct("P"), "template-argument", intended="template-arg")
    case(GRID + "template-keyword", "template template argument, no list / " + col, exp("reject", "accept"),
         kw_verdict(r, True))
    q = Qual(ClassType("Has"), "make", False, kwd, (INT,))
    case(GRID + "template-keyword", "independent qualifier / " + col, "accept",
         kw_verdict(keywords(q, nocls, intended="value"), True))
    case(GRID + "template-keyword", "independent object / " + col, "accept",
         kw_verdict(keywords(Member(Id("h"), "get", False, kwd, (INT,)), nocls), True))
    q = Qual(T, "plain", False, kwd)
    r = keywords(q, ft_, intended="value")
    case(GRID + "template-keyword", "member that is not a template / " + col, exp("accept", "reject"),
         kw_verdict(r, True, at_instantiation(q, "value", r.reads_as)))
    m = Member(Id("t"), "field", False, kwd)
    r = keywords(m, ft_)
    case(GRID + "template-keyword", "data member that is not a template / " + col, exp("accept", "split"),
         kw_verdict(r, True, at_instantiation(m, "value", r.reads_as)))

SAME = AliasTemplate(("U",), Spec(BX, (TParam("U"),)))
PTR = AliasTemplate(("U",), Spec(BX, (Ptr(TParam("U")),)))
CI_ROWS = [("Box", Injected(), ("accept", "reject*", "accept")), ("Box<T>", Spec(BX, (T,)), ("accept", "reject*", "accept")),
           ("::Box<T>", Spec(BX, (T,)), ("accept", "reject*", "accept")),
           ("Same<T>", AliasSpec("Same", (T,)), ("split", "split", "split")),
           ("Box<T*>", Spec(BX, (Ptr(T),)), ("reject", "accept", "accept")),
           ("Pointer<T>", AliasSpec("Pointer", (T,)), ("reject", "accept", "accept")),
           ("Box<const T>", Spec(BX, (CV(T),)), ("reject", "accept", "accept"))]
for row, self_, (e1, e2, e3) in CI_ROWS:
    c1 = ct(members={"type": MemberDecl("alias", INT)}, alias_templates={"Same": SAME, "Pointer": PTR})
    case(GRID + "current-instantiation", row + " / member type without typename", e1,
         kw_verdict(keywords(Qual(self_, "type"), c1, "local", declarator_name="v"), False))
    c2 = ct(alias_templates={"Same": SAME, "Pointer": PTR}, body=[Construct("name", QualId(self_, "missing"))])
    # reject* : every compiler but gcc-14 rejects; DEP-30 lets an implementation accept (Q-0228)
    got = by_strength(pre_checks(c2)[0][2])
    case(GRID + "current-instantiation", row + " / missing member, no dependent base",
         "split" if e2 == "reject*" else e2, got)
    if e2 == "reject*":
        case(GRID + "current-instantiation", row + " / missing member, no dependent base: may (DEP-30, DEP-17)",
             "either", got, "the component should say may")
    c3 = ct(bases=(BT,), alias_templates={"Same": SAME, "Pointer": PTR})
    case(GRID + "current-instantiation", row + " / missing member, dependent base", e3,
         bind_verdict(binding(QualId(self_, "missing"), c3)))

# ----------------------------------------------------------------- INST-1 to INST-13
def ins(e, use, **kw):
    v = instantiates(e, use, **kw)
    return "either" if v == "unspecified" else v


def err_if(v):
    """A class body or function body that is not valid: reject if instantiated."""
    if v == "either":
        return "either"
    return "reject" if v else "accept"


case("classes/virtual/override-dependent-base-pointer", "a pointer does not instantiate", "accept", err_if(ins("class", "pointer-type")))
for t, use in (("class-base-conversion", "base-conversion"), ("class-call-return", "call-returning"),
               ("class-delete", "delete"), ("class-function-definition-parameter", "function-definition-param"),
               ("class-function-definition-return", "function-definition-return"), ("class-member-access", "member-access-pointer"),
               ("class-object", "object"), ("class-qualified-name", "qualified-member-name"), ("class-sizeof", "sizeof"),
               ("class-deduction-from-derived-error", "deduction-base-search")):
    case(I + t, use + " instantiates the class", "reject", err_if(ins("class", use)))
case(I + "class-deduction-from-derived", "deduction searches the bases", True, ins("class", "deduction-base-search"))
case(I + "class-data-member-function-type", "Box<F> b; the member's type is a function type", "reject",
     "reject" if ins("class", "object") and error_effect("data-member-type") == "hard" else "accept")
case(I + "class-static-member-function-type", "static T value with T = int()", "reject",
     "reject" if ins("class", "object") and error_effect("static-data-member") == "hard" else "accept")
case(I + "class-member-declaration-fails", "void set(typename T::type)", "reject",
     "reject" if error_effect("member-function") == "hard" else "accept")
case(I + "member-template-pointer-to-member-non-class", "the member template's declaration", "reject",
     "reject" if error_effect("member-template") == "hard" else "accept")
case(I + "class-unscoped-enumeration", "enum Kind { first = T::nothing }", "reject",
     "reject" if made_with_class("unscoped-enum") == "yes" else "accept")
case(I + "class-scoped-enumeration-unused", "enum class Kind { first = T::nothing }", "accept",
     "reject" if made_with_class("scoped-enum") == "yes" else "accept")
case(I + "class-defined-after-pointer", "Later<int>* before the definition", "accept", err_if(ins("class", "pointer-type")))
case(I + "class-deleted-member-specialized", "a deleted body waits", "accept",
     "reject" if made_with_class("deleted-function-body") == "yes" else "accept")
case(I + "class-inline-static-member-unused", "inline static initializer waits", "accept",
     "reject" if made_with_class("static-inline-initializer") == "yes" else "accept")
for part in ("member-function-body", "nested-class", "member-template", "default-argument", "exception-specification",
             "static-data-member", "default-member-initializer"):
    case(I + "class-member-definitions-unused", part, "accept", "reject" if made_with_class(part) == "yes" else "accept")
for t in ("class-overload-two-candidates", "class-overload-void-pointer"):
    case(I + t, "overload resolution that needs no definition", "reject", err_if(ins("class", "overload-resolution")))
for use, lab in (("pointer-type", "Box<int>*"), ("reference-type", "Box<int>&"), ("function-declaration", "Box<int> make();"),
                 ("alias", "using Name = Box<int>"), ("template-argument", "Holder<Box<int>>")):
    case(I + "class-pointer-only", lab, "accept", err_if(ins("class", use)))
case(I + "default-argument-each-own", "y given, z used", "run", err_if(ins("default-argument", "call-gives")))
case(I + "default-argument-explicit-instantiation", "explicit instantiation", "accept",
     err_if(ins("default-argument", "explicit-instantiation")))
case(I + "default-argument-explicit-specialization", "call of the specialization leaves it out", "reject",
     err_if(ins("default-argument", "call-omits")))
case(I + "default-argument-not-used", "pick(1, 2)", "accept", err_if(ins("default-argument", "call-gives")))
case(I + "default-argument-used", "pick(1)", "reject", err_if(ins("default-argument", "call-omits")))
case(I + "default-member-initializer-aggregate", "Pair<int> p{1, 2}", "accept",
     err_if(ins("default-member-initializer", "aggregate-gives")))
case(I + "default-member-initializer-aggregate-left-out", "Pair<int> p{1}", "reject",
     err_if(ins("default-member-initializer", "aggregate-omits")))
case(I + "default-member-initializer-not-used", "Box(int) : width(1)", "accept",
     err_if(ins("default-member-initializer", "constructor-gives")))
case(I + "default-member-initializer-used", "Box() {}", "reject", err_if(ins("default-member-initializer", "constructor-omits")))
case(I + "explicit-class-members", "template struct Box<int>", "reject", err_if(ins("function", "explicit-instantiation")))
case(I + "explicit-member-class", "template struct Box<int>::Inner", "reject", err_if(ins("function", "explicit-instantiation")))
case(I + "explicit-declaration-inline-still-made", "Box<int>{}.get()", "reject", err_if(ins("function", "call")))
case(I + "friend-defined-per-specialization", "friend hook(Box)", "run",
     "reject" if friend_redefinition(True, 2) else "accept")
case(I + "friend-defined-twice", "friend hook()", "reject", "reject" if friend_redefinition(False, 2) else "accept")
case(I + "function-address", "&make<int>", "reject", err_if(ins("function", "address")))
case(I + "function-called", "make<int>()", "reject", err_if(ins("function", "call")))
case(I + "member-function-called", "Box<int>{}.get()", "reject", err_if(ins("function", "call")))
case(I + "virtual-called-through-pointer", "p->get()", "reject", err_if(ins("function", "call")))
case(I + "function-deduced-return", "decltype(make<int>()) with auto", "reject",
     err_if(ins("function", "decltype-call", deduced_return=True)))
case(I + "function-unevaluated-braced", "decltype(int{fail<int>()})", "reject",
     err_if(ins("function", "decltype-call", constexpr_braced=True)))
case(I + "function-unevaluated-braced-untaken", "decltype(char{true ? 1 : fail<int>()})", "reject",
     err_if(ins("function", "decltype-call", constexpr_braced=True)))
for use in ("decltype-call", "sizeof-call", "noexcept-call"):
    case(I + "function-unevaluated", use, "accept", err_if(ins("function", use)))
case(I + "function-unevaluated-parentheses", "decltype(char(... fail<int>()))", "accept", err_if(ins("function", "decltype-call")))
case(I + "function-unevaluated-parentheses", "decltype(fail<int>() + 1)", "accept", err_if(ins("function", "decltype-call")))
case(I + "function-unevaluated-sizeof-constant", "sizeof(fail<int>() + 1)", "accept", err_if(ins("function", "sizeof-call")))
LP = local_class_parts()
for t, part in (("local-class-default-argument", "default-argument"), ("local-class-exception-specification", "exception-specification"),
                ("local-class-member", "member-function-body"), ("local-class-member-initializer", "default-member-initializer")):
    case(I + t, part + " made with the function", "reject", "reject" if LP[part] else "accept")
case(I + "local-class-requires-clause-waits", "the requires clause waits", "run", "reject" if LP["requires-clause"] else "accept")
case(I + "local-enumeration", "a local enumeration", "reject", err_if(ins("local-class-parts", "function-instantiated")))
case(I + "static-member-not-initialized", "Box<int> b does not run the initializer", "run",
     err_if(ins("static-member", "object-of-class")))
case(I + "static-member-used", "Box<int>::count", "reject", err_if(ins("static-member", "read")))
case(I + "variable-template-unevaluated", "decltype(zero<int>)", "accept", err_if(ins("variable-template", "decltype")))
case(I + "variable-template-used", "zero<int>", "reject", err_if(ins("variable-template", "read")))
for t in ("virtual-new-expression", "virtual-object", "virtual-sizeof", "virtual-temporary"):
    exp = "accept" if t == "virtual-sizeof" else "reject"
    case(I + t, "a virtual function no use names", exp, err_if(ins("virtual-function-unused", "")))

LEFT_OUT = [
    (I + "class-declared-not-defined", "INST-4: the component has no call for a class template that is not defined"),
    (I + "member-partial-specializations-collide", "INST-6 second paragraph: no model of member partial specializations"),
    (I + "local-entities-per-specialization", "INST-36: no model of local statics and closure types"),
    (I + "local-entities-thread-local-and-nested-lambda", "INST-36: as above"),
    (I + "local-class-requires-clause-checked-at-use", "CNS-12: constraint satisfaction at the use"),
    (I + "local-class-requires-clause-substitution-failure-call", "CNS-12: as above"),
    (I + "local-class-requires-clause-substitution-failure", "CNS-12 and CPT: as above"),
]

if __name__ == "__main__":
    right = 0
    by_src = {}
    wrong = []
    for src, cid, exp, got, note in CASES:
        good = ok(exp, got)
        right += good
        key = "grids" if "grids/" in src else ("dependent-names" if src.startswith(D) else "instantiation")
        a, b = by_src.get(key, (0, 0))
        by_src[key] = (a + 1, b + good)
        if not good:
            wrong.append((src, cid, exp, got))
    tests = {s for s, *_ in CASES}
    print(f"{right} of {len(CASES)} assertions right, from {len(tests)} tests and grids")
    for k, (a, b) in sorted(by_src.items()):
        print(f"  {k}: {b} of {a}")
    print("\nWRONG")
    for w in wrong:
        print("  ", *w, sep=" | ")
    print("\nLEFT OUT")
    for s, why in LEFT_OUT:
        print("  ", s, "|", why)
