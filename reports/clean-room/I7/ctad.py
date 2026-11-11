#!/usr/bin/env python3
"""I7: class template argument deduction, from the Tidal rules
(templates/class-template-deduction.md, CTAD-P1 and its rules).

The component uses I5's deduction (reports/clean-room/I5/deduce.py) as its
callback: deduce() for one guide, ics() for conversion ranks, and
at_least_as_specialized() for partial ordering. It changes one function of I5,
is_forwarding(), so that a class parameter T&& in a guide from a constructor
is no forwarding reference (CTAD-8, CTAD-28).

Entry points:
  check_place(place, alias=None, has_list=False) -> None | Fail      (CTAD-1, 2, 4, 15)
  guides_of(name, env, init)                    -> list of Guide     (CTAD-5 to 16, 27 to 29)
  ctad(name, init, env)                         -> Result | Fail     (CTAD-17 to 26)
  ctad_declarators(name, inits, env)            -> Result | Fail     (CTAD-3)
  check_user_guide(guide, env)                  -> None | Fail       (CTAD-12, CTAD-13)
"""
import importlib.util
import os
from dataclasses import dataclass, field, replace
from typing import Callable

_here = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("i5deduce", os.path.join(_here, "..", "I5", "deduce.py"))
D = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(D)

from_i5 = ["Prim", "Ptr", "LRef", "RRef", "Arr", "Cls", "TParam", "Expand", "Qualified", "Val",
           "CParam", "TP", "FP", "Template", "Env", "ClassDef", "Arg", "Braced", "Deduced",
           "Failure", "HardError", "INT", "C", "V", "cvs", "strip_cv", "tparams_in"]
for _n in from_i5:
    globals()[_n] = getattr(D, _n)

# ---------------------------------------------------------------- CTAD-8: class parameters are no forwarding references
_orig_fw = D.is_forwarding


def _is_forwarding(P, tmpl):
    if not _orig_fw(P, tmpl):
        return False
    return P.to.name not in getattr(tmpl, "class_params", ())


D.is_forwarding = _is_forwarding

# A braced list converts to an aggregate class of the world in use (LIST-3). I5's ics_braced
# looks for a converting constructor only, so the component adds the aggregate case.
_orig_icsb = D.ics_braced
AGGREGATES = set()


def _ics_braced(arg, P, env):
    Q = P.to if isinstance(P, (D.LRef, D.RRef)) else D.strip_cv(P)
    if isinstance(Q, D.Cls) and Q.name in AGGREGATES and not isinstance(P, D.LRef) or \
            isinstance(P, D.LRef) and C in D.cv_of(P.to) and isinstance(Q, D.Cls) and Q.name in AGGREGATES:
        if len(arg.elems) == 1 and isinstance(arg.elems[0], D.Arg) and D.strip_cv(arg.elems[0].type) == D.strip_cv(Q):
            return D.EXACT
        return D.USER
    return _orig_icsb(arg, P, env)


D.ics_braced = _ics_braced


# ---------------------------------------------------------------- the tree
@dataclass
class Ctor:
    """A constructor of the primary class template. params use the class's
    TParams and the constructor's own tparams."""
    params: list
    tparams: list = field(default_factory=list)  # a constructor template's own parameters
    explicit: object = False                      # bool, or fn(bindings) -> bool (conditional explicit)
    constraint: Callable = None                   # fn(bindings) -> bool  (CTAD-29)
    deleted: bool = False                          # for step 5 (CTAD-26, DEL-1)
    access: str = "public"                         # for step 5 (CTAD-26, ACC-3)
    label: str = ""


@dataclass
class UserGuide:
    """A deduction_guide. result is a Cls of the class template."""
    template: str
    params: list
    result: object
    tparams: list = field(default_factory=list)
    explicit: bool = False
    pos: int = 0            # position in the translation unit (CTAD-11)
    label: str = ""


@dataclass
class Element:
    """An element of an aggregate class template, in declaration order: bases first."""
    type: object            # declared type; Expand(TParam) for a pack of bases
    kind: str = "member"    # 'member' or 'base'
    name: str = ""
    has_default: bool = False  # a default member initializer


@dataclass
class ClassTemplate:
    name: str
    tparams: list
    defined: bool = True
    ctors: list = field(default_factory=list)      # declared constructors; [] declares none
    aggregate: bool = False
    elements: list = field(default_factory=list)   # for an aggregate
    inherits: list = field(default_factory=list)   # base Cls types with `using B::B;` (CTAD-16)
    constraint: Callable = None                    # fn(bindings) -> bool (CTAD-29)


@dataclass
class AliasTemplate:
    name: str
    tparams: list
    target: object          # a Cls (or alias Cls) the alias stands for
    constraint: Callable = None
    other_type: bool = False          # the alias stands for a pointer or other non-class type (CTAD-15)
    dependent_qualifier: bool = False  # the type names the class template after a dependent qualifier


@dataclass
class World:
    """The class templates, aliases, user guides, and I5's Env for classes."""
    templates: dict = field(default_factory=dict)
    aliases: dict = field(default_factory=dict)
    guides: list = field(default_factory=list)
    env: object = None

    def __post_init__(self):
        if self.env is None:
            self.env = Env()


@dataclass
class Init:
    """form: 'none', 'paren' (direct), 'copy' (= v), 'list' (direct-list {}), 'copy-list' (= {}).
    args: I5 Arg / Braced. designators: member names for a designated list."""
    form: str = "paren"
    args: list = field(default_factory=list)
    designators: list = None
    use_pos: int = 10 ** 9   # position of the use, for CTAD-11


@dataclass
class Guide:
    tmpl: object            # an I5 Template
    result: object          # Cls, in terms of the guide's parameters
    origin: str             # 'ctor', 'ctor-template', 'default', 'copy', 'aggregate', 'user'
    explicit: object = False
    inherited: bool = False
    alias: object = None    # the AliasTemplate whose CTAD-28 check applies
    ctor: object = None
    label: str = ""
    is_template: bool = True
    initlist: bool = False  # the guide's parameters make an initializer-list guide (ILC-3 phase 1)


@dataclass
class Result:
    type: object
    guide: str
    kind = "deduced"


@dataclass
class Fail:
    rule: str
    why: str
    kind = "fail"


# ---------------------------------------------------------------- helpers
def cls_of(name, tparams):
    """The class template with its own parameters as arguments."""
    args = []
    for tp in tparams:
        a = TParam(tp.name) if tp.kind == "type" else CParam(tp.name) if tp.kind == "const" else D.TTParam(tp.name)
        args.append(Expand(a) if tp.pack else a)
    return Cls(name, tuple(args), is_template=True)


def subst(t, b, env):
    return D.subst(t, b, env)


def make_template(name, tparams, params, ret, class_params=(), constraint=None, explicit=None):
    t = Template(name, list(tparams), list(params), ret)
    t.class_params = set(class_params)
    t.constraint = constraint
    return t


def is_initlist_param(fp):
    p = fp.type
    q = p.to if isinstance(p, (LRef, RRef)) else p
    q = strip_cv(q)
    return isinstance(q, Cls) and q.name == "std::initializer_list"


def both(c1, c2):
    if c1 is None:
        return c2
    if c2 is None:
        return c1
    return lambda b: c1(b) and c2(b)


# ---------------------------------------------------------------- step 1: places
OK_PLACES = {"variable", "variable-no-init", "static-member-init", "range-for", "condition",
             "new-init", "functional-cast", "constant-param"}


def check_place(place, alias=None, has_list=False):
    """CTAD-1, CTAD-2, CTAD-4, CTAD-15. place is a word from OK_PLACES or another place,
    such as 'pointer', 'reference', 'array', 'function-param', 'return', 'nonstatic-member',
    'new-no-init', 'static-member-no-init'."""
    if has_list:
        return None  # CTAD-4: a template-id; no deduction
    if alias is not None and (alias.other_type or alias.dependent_qualifier):
        return Fail("CTAD-15", f"{alias.name} is not an alias of CTAD-14")
    if place in OK_PLACES:
        return None
    return Fail("CTAD-2", f"a class template name with no argument list in place '{place}'")


# ---------------------------------------------------------------- step 2: guides
def ctor_guides(ct):
    """CTAD-5 and CTAD-6."""
    res = cls_of(ct.name, ct.tparams)
    cparams = [tp.name for tp in ct.tparams]
    out = []
    if ct.defined and ct.ctors:
        for c in ct.ctors:
            t = make_template(ct.name, ct.tparams + c.tparams, c.params, res, cparams,
                              both(ct.constraint, c.constraint))
            out.append(Guide(t, res, "ctor-template" if c.tparams else "ctor", c.explicit, ctor=c,
                             label=c.label or f"ctor({', '.join(repr(p.type) for p in c.params)})",
                             initlist=bool(c.params) and is_initlist_param(c.params[0])
                             and all(p.default for p in c.params[1:])))
    else:
        t = make_template(ct.name, ct.tparams, [], res, cparams, ct.constraint)
        out.append(Guide(t, res, "default", False, label="no-parameter guide"))
    return out


def copy_guide(ct):
    """CTAD-7."""
    res = cls_of(ct.name, ct.tparams)
    t = make_template(ct.name, ct.tparams, [FP(res)], res, [tp.name for tp in ct.tparams])
    return Guide(t, res, "copy", False, label="copy candidate")


def user_guides(name, world, use_pos):
    """CTAD-11: the user guides declared before the use."""
    out = []
    for g in world.guides:
        if g.template != name or g.pos >= use_pos:
            continue
        t = make_template(name, g.tparams, g.params, g.result)
        out.append(Guide(t, g.result, "user", g.explicit, label=g.label or "user guide",
                         is_template=bool(g.tparams),
                         initlist=bool(g.params) and is_initlist_param(g.params[0])
                         and all(p.default for p in g.params[1:])))
    return out


def dependent(t, names):
    return bool(tparams_in(t) & names) if names else False


def aggregate_guide(ct, init, world):
    """CTAD-9, CTAD-10, CTAD-27. Returns a Guide, a Fail for a fixed pack that disagrees, or None."""
    if not ct.defined or not ct.aggregate:
        return None
    if any(g.template == ct.name and g.pos < init.use_pos for g in world.guides):
        return None
    if init.form not in ("list", "copy-list", "paren") or not init.args:
        return None
    names = {tp.name for tp in ct.tparams}
    packs = {tp.name for tp in ct.tparams if tp.pack}
    braces = init.form in ("list", "copy-list")
    elems = list(ct.elements)
    if init.designators is not None:
        # LIST-5: each designated member takes its element, in declaration order
        by_name = {e.name: e for e in elems}
        params = []
        for d in init.designators:
            if d not in by_name:
                return None
            params.append(FP(by_name[d].type))
        return _agg(ct, params)
    params = []
    args = list(init.args)
    i = 0
    fixed_pack = None
    for k, e in enumerate(elems):
        is_last = k == len(elems) - 1
        if isinstance(e.type, Expand):  # a pack of bases (CTAD-27)
            pname = e.type.pattern.name
            if fixed_pack is not None and pname in fixed_pack:
                bases = fixed_pack[pname]
                for j, bt in enumerate(bases):
                    if i < len(args):
                        a = args[i]
                        if not isinstance(a, Braced) and strip_cv(a.type) != strip_cv(bt):
                            return Fail("CTAD-27", f"{a.type!r} stands where the fixed base {bt!r} is")
                        params.append(FP(bt))
                        i += 1
                    else:
                        params.append(FP(bt, default=True))
                continue
            if is_last:
                if i < len(args):
                    params.append(FP(e.type.pattern, pack=True))
                    i = len(args)
                continue
            continue  # a pack before other elements takes no element
        if i >= len(args):
            break
        a = args[i]
        t = e.type
        if isinstance(t, Arr):
            if isinstance(t.bound, CParam) or (dependent(t.elem, names) and not isinstance(t.bound, int)):
                # no elision: one element for the whole array
                if isinstance(a, Braced):
                    params.append(FP(RRef(t)))
                elif isinstance(a.type, Arr):  # a string literal
                    params.append(FP(LRef(Arr(D.add_cv(t.elem, cvs(C)), t.bound))))
                else:
                    return None
                i += 1
                continue
            # fixed bound: braces elided, also for a string literal (Q-0277)
            if isinstance(a, Braced) and braces:
                params.append(FP(RRef(t)))
                i += 1
                continue
            n = t.bound if isinstance(t.bound, int) else 1
            for _ in range(n):
                if i >= len(args):
                    break
                params.append(FP(t.elem))
                i += 1
            continue
        params.append(FP(t))
        i += 1
        # an element that fixes the length of a later pack (CTAD-27)
        if packs and not isinstance(a, Braced) and isinstance(t, Cls):
            b = {"__packs__": set(packs)}
            try:
                D.unify(t, strip_cv(a.type), b, world.env, False)
            except D.PairFail:
                b = {}
            for p in packs:
                if p in b and isinstance(b[p], tuple):
                    fixed_pack = fixed_pack or {}
                    fixed_pack[p] = b[p]
    if i < len(args):
        return None  # CTAD-10: an element has no member to initialize
    return _agg(ct, params)


def _agg(ct, params):
    res = cls_of(ct.name, ct.tparams)
    t = make_template(ct.name, ct.tparams, params, res, [tp.name for tp in ct.tparams], ct.constraint)
    return Guide(t, res, "aggregate", False, label="aggregate candidate")


def class_guides(ct, init, world, with_aggregate=True):
    gs = ctor_guides(ct) + [copy_guide(ct)] + user_guides(ct.name, world, init.use_pos)
    if with_aggregate:
        a = aggregate_guide(ct, init, world)
        if isinstance(a, Fail):
            gs.append(Guide(None, None, "aggregate", label="aggregate candidate", explicit=a))
        elif a is not None:
            gs.append(a)
    for base in ct.inherits:
        gs += inherited_guides(ct, base, init, world)
    return gs


# ---------------------------------------------------------------- CTAD-14: guides through an alias
def match_result(P, A, b, fixed):
    """Match a guide's result P (guide parameters free) against the alias type A.
    Alias parameters in A are opaque. b maps guide parameter -> type in alias parameters."""
    if isinstance(P, TParam) and not P.cv and P.name not in fixed:
        if P.name in b and b[P.name] != A:
            raise D.PairFail("CTAD-14", f"{P.name} is {b[P.name]!r} and {A!r}")
        b[P.name] = A
        return
    if isinstance(P, CParam) and P.name not in fixed:
        if P.name in b and b[P.name] != A:
            raise D.PairFail("CTAD-14", "values differ")
        b[P.name] = A
        return
    if isinstance(P, Expand) and isinstance(P.pattern, (TParam, CParam)):
        b[P.pattern.name] = A
        return
    if type(P) is not type(A):
        raise D.PairFail("CTAD-14", f"{P!r} against {A!r}")
    if isinstance(P, Cls):
        if P.name != A.name:
            raise D.PairFail("CTAD-14", f"{P!r} against {A!r}")
        pa, aa = list(P.args), list(A.args)
        for k, p in enumerate(pa):
            if isinstance(p, Expand):
                b[p.pattern.name] = tuple(aa[k:])
                return
            if k >= len(aa):
                raise D.PairFail("CTAD-14", "too few arguments")
            match_result(p, aa[k], b, fixed)
        if len(aa) > len(pa):
            raise D.PairFail("CTAD-14", "too many arguments")
        return
    if isinstance(P, (Ptr, LRef, RRef)):
        match_result(P.to, A.to, b, fixed)
        return
    if P != A:
        raise D.PairFail("CTAD-14", f"{P!r} against {A!r}")


def adapt(g, alias_tparams, target, env, alias_name, alias_obj, inherited=False, derived=None):
    """Make a guide of an alias (or of a derived class, CTAD-16) from the guide g of the class."""
    if g.tmpl is None:
        return None
    gt = g.tmpl
    # rename the guide's parameters so that they do not meet the alias's names
    ren = {}
    for tp in gt.tparams:
        ren[tp.name] = tp.name + "'"
    rb = {}
    for tp in gt.tparams:
        nn = ren[tp.name]
        if tp.kind == "type":
            rb[tp.name] = (TParam(nn),) if tp.pack else TParam(nn)
        elif tp.kind == "const":
            rb[tp.name] = (CParam(nn),) if tp.pack else CParam(nn)
        else:
            rb[tp.name] = D.TmplName(nn)

    def r(t):
        return _rename(t, ren)
    result = r(g.result)
    b = {}
    try:
        match_result(result, target, b, set())
    except D.PairFail:
        return None  # CTAD-14: the result cannot match the alias
    used_alias = set()
    for v in b.values():
        for x in (v if isinstance(v, tuple) else (v,)):
            used_alias |= tparams_in(x) & {tp.name for tp in alias_tparams}
    new_tparams = [tp for tp in alias_tparams if tp.name in used_alias or inherited]
    left = [replace(tp, name=ren[tp.name], default=None if tp.default is None else r(tp.default))
            for tp in gt.tparams if ren[tp.name] not in b]
    sb = {}
    for k, v in b.items():
        sb[k] = v
    params = []
    for fp in gt.params:
        pt = r(fp.type)
        pt = _subst_partial(pt, sb)
        params.append(FP(pt, fp.default, fp.pack))
    new_result = _subst_partial(result, sb)
    # CTAD-28 (Q-0278): a parameter that stands for a class parameter stays one
    src_cps = {ren[n] for n in getattr(gt, "class_params", ())}
    cps = {n for n in src_cps if n not in b}
    for n in src_cps & set(b):
        for x in (b[n] if isinstance(b[n], tuple) else (b[n],)):
            cps |= tparams_in(x) & {tp.name for tp in alias_tparams}
    t = make_template(alias_name, new_tparams + left, params, new_result, cps, None)
    ng = Guide(t, new_result, g.origin, g.explicit, inherited or g.inherited, alias_obj, g.ctor,
               g.label + (" (inherited)" if inherited else f" via {alias_name}"), True, g.initlist)
    ng.derived = derived
    return ng


def _rename(t, ren):
    if isinstance(t, TParam):
        return TParam(ren.get(t.name, t.name), t.cv)
    if isinstance(t, CParam):
        return CParam(ren.get(t.name, t.name))
    if isinstance(t, Expand):
        return Expand(_rename(t.pattern, ren))
    if isinstance(t, Cls):
        return Cls(t.name, tuple(_rename(a, ren) for a in t.args), t.cv, t.is_template)
    if isinstance(t, Ptr):
        return Ptr(_rename(t.to, ren), t.cv)
    if isinstance(t, LRef):
        return LRef(_rename(t.to, ren))
    if isinstance(t, RRef):
        return RRef(_rename(t.to, ren))
    if isinstance(t, Arr):
        return Arr(_rename(t.elem, ren), _rename(t.bound, ren) if isinstance(t.bound, CParam) else t.bound)
    if isinstance(t, D.Qualified):
        return D.Qualified(_rename(t.scope, ren), t.member, t.cv)
    return t


def _subst_partial(t, b):
    if isinstance(t, TParam):
        if t.name in b:
            v = b[t.name]
            return D.add_cv(v, t.cv) if t.cv and not isinstance(v, tuple) else v
        return t
    if isinstance(t, CParam):
        return b.get(t.name, t)
    if isinstance(t, Expand):
        if isinstance(t.pattern, (TParam, CParam)) and t.pattern.name in b:
            return b[t.pattern.name]
        return Expand(_subst_partial(t.pattern, b))
    if isinstance(t, Cls):
        args = []
        for a in t.args:
            s = _subst_partial(a, b)
            if isinstance(a, Expand) and isinstance(s, tuple):
                args.extend(s)
            else:
                args.append(s)
        return Cls(t.name, tuple(args), t.cv, t.is_template)
    if isinstance(t, Ptr):
        return Ptr(_subst_partial(t.to, b), t.cv)
    if isinstance(t, LRef):
        return LRef(_subst_partial(t.to, b))
    if isinstance(t, RRef):
        return RRef(_subst_partial(t.to, b))
    if isinstance(t, Arr):
        bd = b.get(t.bound.name, t.bound) if isinstance(t.bound, CParam) else t.bound
        if isinstance(bd, D.Val):
            bd = bd.value
        return Arr(_subst_partial(t.elem, b), bd)
    return t


def alias_chain(al, world):
    """Resolve an alias of an alias to its class template. Returns (class name, target in al's params)."""
    t = al.target
    seen = 0
    while t.name in world.aliases and seen < 20:
        inner = world.aliases[t.name]
        b = {}
        for tp, a in zip(inner.tparams, t.args):
            b[tp.name] = a
        t = _subst_partial(inner.target, b)
        seen += 1
    return t.name, t


def alias_guides(al, init, world):
    """CTAD-14."""
    cname, target = alias_chain(al, world)
    ct = world.templates[cname]
    out = []
    for g in class_guides(ct, init, world):
        ng = adapt(g, al.tparams, target, world.env, al.name, al)
        if ng is not None:
            out.append(ng)
    return out


def inherited_guides(ct, base, init, world):
    """CTAD-16: the base type acts as an alias of the derived parameters."""
    bt = world.templates.get(base.name)
    if bt is None:
        return []
    out = []
    pseudo = AliasTemplate(f"{ct.name}:{base.name}", ct.tparams, base, None)
    for g in ctor_guides(bt) + [copy_guide(bt)] + user_guides(bt.name, world, init.use_pos):
        if g.origin == "default":
            continue  # a default constructor is not inherited (DER-8)
        ng = adapt(g, ct.tparams, base, world.env, ct.name, None, inherited=True, derived=ct)
        if ng is not None:
            ng.result = cls_of(ct.name, ct.tparams)
            ng.tmpl.ret = ng.result
            out.append(ng)
    return out


def guides_of(name, world, init=None):
    init = init or Init()
    if name in world.aliases:
        return alias_guides(world.aliases[name], init, world)
    return class_guides(world.templates[name], init, world)


# ---------------------------------------------------------------- step 3: choose
def run_guide(g, args, world):
    """Deduce one guide with I5. Returns (Deduced) or None, and the result type."""
    d = D.deduce(g.tmpl, args, (), world.env)
    if not isinstance(d, Deduced) or not d.viable:
        return d, None
    try:
        res = D.subst(g.result, d.args, world.env)
    except (D.SubstFail, D.Hard) as e:
        return Failure("CTAD-14", str(e)), None
    if g.alias is not None:  # CTAD-28
        ok = alias_check(g.alias, res, world)
        if isinstance(ok, Fail):
            return Failure(ok.rule, ok.why), None
    if getattr(g, "derived", None) is not None:
        # CTAD-16: each derived parameter needs a value, from the base or a default
        ct = g.derived
        dargs = {}
        for tp in ct.tparams:
            if tp.name in d.args:
                dargs[tp.name] = d.args[tp.name]
            elif tp.default is not None:
                dargs[tp.name] = D.subst(tp.default, dargs, world.env) if not isinstance(tp.default, Val) else tp.default
            elif tp.pack:
                dargs[tp.name] = ()
            else:
                return Failure("CTAD-16", f"no guide gives {tp.name}"), None
        res = D.subst(cls_of(ct.name, ct.tparams), dargs, world.env)
        if ct.constraint is not None and not ct.constraint(dargs):
            return Failure("CTAD-29", "the class's constraints are not satisfied"), None
    return d, res


def alias_check(al, res, world):
    """CTAD-28: the result is a specialization of the alias, and the alias's constraints hold."""
    _, target = alias_chain(al, world)
    tmpl = Template(al.name, list(al.tparams), [FP(target)], D.VOID)
    d = D.deduce(tmpl, [Arg(res, "lvalue")], (), world.env)
    if not isinstance(d, Deduced):
        return Fail("CTAD-28", f"{res!r} is not a specialization of {al.name}")
    try:
        if D.subst(target, d.args, world.env) != strip_cv(res):
            return Fail("CTAD-28", f"{res!r} is not a specialization of {al.name}")
    except (D.SubstFail, D.Hard):
        return Fail("CTAD-28", f"{res!r} is not a specialization of {al.name}")
    if al.constraint is not None and not al.constraint(d.args):
        return Fail("CTAD-28", f"the constraints of {al.name} are not satisfied")
    return None


def is_explicit(g, d):
    e = g.explicit
    if callable(e):
        return bool(e(d.args))
    return bool(e)


def ordered(g1, g2, nargs, env):
    """OVL-28 with ORD-5: only the parameters that the call gives arguments take part."""
    def cut(t):
        ps = []
        n = 0
        for fp in t.params:
            if fp.pack:
                ps.append(fp)
                break
            if n >= nargs:
                break
            ps.append(fp)
            n += 1
        return Template(t.name, t.tparams, ps, t.ret)
    return D.at_least_as_specialized(cut(g1.tmpl), cut(g2.tmpl), env)


def beats(v, w, nargs, env):
    g1, d1, _ = v
    g2, d2, _ = w
    r1, r2 = d1.ranks, d2.ranks
    if any(a > b for a, b in zip(r1, r2)):
        return False
    if any(a < b for a, b in zip(r1, r2)):
        return True
    # OVL-27: a guide that is not a template wins over a template
    if not g1.is_template and g2.is_template:
        return True
    if g1.is_template and not g2.is_template:
        return False
    if g1.is_template and g2.is_template:  # OVL-28
        a = ordered(g1, g2, nargs, env)
        b = ordered(g2, g1, nargs, env)
        if a and not b:
            return True
        if b and not a:
            return False
    # CTAD-19
    same = list(d1.ptypes[:nargs]) == list(d2.ptypes[:nargs])
    if same and not g1.inherited and g2.inherited:
        return True
    if same and g1.inherited and not g2.inherited:
        return False
    # CTAD-20
    if g1.origin == "user" and g2.origin != "user":
        return True
    if g2.origin == "user" and g1.origin != "user":
        return False
    # CTAD-21
    if g1.origin == "copy" and g2.origin != "copy":
        return True
    if g2.origin == "copy" and g1.origin != "copy":
        return False
    # CTAD-22
    if g1.origin == "ctor" and g2.origin == "ctor-template":
        return True
    return False


def choose(gs, args, init, world, copy_init):
    viable = []
    fails = []
    for g in gs:
        if g.tmpl is None:
            fails.append((g.label, g.explicit.rule, g.explicit.why))
            continue
        d, res = run_guide(g, args, world)
        if isinstance(d, D.HardError):
            return Fail(d.rule, d.why)
        if not isinstance(d, Deduced) or not d.viable:
            fails.append((g.label, getattr(d, "rule", "OVL-22"), getattr(d, "why", "")))
            continue
        if copy_init and is_explicit(g, d):
            continue  # CTAD-17
        viable.append((g, d, res))
    if not viable:
        return None, fails
    best = [v for v in viable if all(v is w or beats(v, w, len(args), world.env) for w in viable)]
    if len(best) != 1:
        return Fail("CTAD-24", "ambiguous: " + ", ".join(v[0].label for v in viable)), fails
    return best[0], fails


def ctad(name, init, world):
    """Steps 3 to 5. name names a class template or an alias template."""
    AGGREGATES.clear()
    AGGREGATES.update(n for n, t in world.templates.items() if t.aggregate)
    if name in world.aliases:
        al = world.aliases[name]
        if al.other_type or al.dependent_qualifier:
            return Fail("CTAD-15", f"{name} is not an alias of a class template specialization")
    elif name not in world.templates:
        return Fail("CTAD-15", f"{name} is not a class template")
    gs = guides_of(name, world, init)
    copy_init = init.form == "copy"
    args = list(init.args)
    chosen = None
    if init.form in ("list", "copy-list"):
        cname = alias_chain(world.aliases[name], world)[0] if name in world.aliases else name
        skip = len(args) == 1 and not isinstance(args[0], Braced) and _spec_of(args[0].type, cname, world)
        if not skip and args:  # notes: an empty list skips phase 1
            il = [g for g in gs if g.initlist]
            if il:
                r, _ = choose(il, [Braced(args)], init, world, False)
                if isinstance(r, Fail):
                    return r
                chosen = r
    if chosen is None:
        r, fails = choose(gs, args, init, world, copy_init)
        if isinstance(r, Fail):
            return r
        if r is None:
            for f in fails:
                if f[1] in ("CTAD-27",):
                    return Fail("CTAD-24", f"no viable guide ({f[1]}: {f[2]})")
            return Fail("CTAD-24", "no viable guide: " + "; ".join(f"{a}: {b}" for a, b, _ in fails))
        chosen = r
    g, d, res = chosen
    if init.form == "copy-list" and is_explicit(g, d):
        return Fail("CTAD-23", f"the chosen guide {g.label} is explicit")
    # step 5 (CTAD-25, CTAD-26)
    if g.ctor is not None:
        if g.ctor.deleted:
            return Fail("CTAD-26", f"{res!r}: the constructor is deleted (DEL-1)")
        if g.ctor.access != "public":
            return Fail("CTAD-26", f"{res!r}: the constructor is {g.ctor.access} (ACC-3)")
    return Result(res, g.label)


def _spec_of(t, cname, world):
    """CTAD-18: a specialization of the class template, or a class derived from one."""
    t = strip_cv(t.to if isinstance(t, (LRef, RRef)) else t)
    if not isinstance(t, Cls):
        return False
    if t.name == cname:
        return True
    try:
        d = world.env.classdef(t)
    except D.Hard:
        return False
    return any(_spec_of(b, cname, world) for b in d.bases)


def ctad_declarators(name, inits, world):
    """CTAD-3: each declarator deduces the same type."""
    types = []
    for i in inits:
        r = ctad(name, i, world)
        if isinstance(r, Fail):
            return r
        types.append(r.type)
    if any(t != types[0] for t in types):
        return Fail("CTAD-3", "the declarators deduce " + ", ".join(map(repr, types)))
    return Result(types[0], "all declarators")


# ---------------------------------------------------------------- user guide checks
def check_user_guide(g, world, others=()):
    """CTAD-12 and CTAD-13."""
    r = g.result
    if not isinstance(r, Cls) or r.name != g.template:
        return Fail("CTAD-12", "the result names another template")
    if r.cv:
        return Fail("CTAD-12", "the result has a cv-qualifier")
    for o in others:
        if o is g:
            continue
        if not g.tparams and not o.tparams and o.template == g.template \
                and [p.type for p in o.params] == [p.type for p in g.params]:
            return Fail("CTAD-12", "two guides that are not templates have the same parameter types")
    deduced = set()
    for fp in g.params:
        deduced |= _deducible(fp.type)
    for tp in g.tparams:
        if tp.name not in deduced and tp.default is None and not tp.pack:
            return Fail("CTAD-13", f"{tp.name} has no default and no deduced position")
    return None


def _deducible(t):
    if isinstance(t, D.Qualified):
        return set()
    if isinstance(t, TParam):
        return {t.name}
    if isinstance(t, CParam):
        return {t.name}
    if isinstance(t, Expand):
        return _deducible(t.pattern)
    if isinstance(t, Cls):
        s = set()
        for a in t.args:
            s |= _deducible(a)
        return s
    if isinstance(t, (Ptr, LRef, RRef)):
        return _deducible(t.to)
    if isinstance(t, Arr):
        s = _deducible(t.elem)
        if isinstance(t.bound, CParam):
            s.add(t.bound.name)
        return s
    if isinstance(t, D.Func):
        s = _deducible(t.ret)
        for p in t.params:
            s |= _deducible(p)
        return s
    return set()
