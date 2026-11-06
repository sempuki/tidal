#!/usr/bin/env python3
"""I5: template argument deduction from a function call, and the substitution
boundary, from the Tidal rules (templates/deduction.md and the pages it names).

Entry points:
  deduce(tmpl, args, explicit=(), env=Env())  -> Deduced | Failure | HardError
  resolve(cands, args, explicit=(), env=Env()) -> Chosen | Reject
  deduce_target(tmpl, target, env)             -> Deduced | Failure   (DED-30)
"""
from dataclasses import dataclass, field
from typing import Callable

C, V = "const", "volatile"
NOCV = frozenset()


def cvs(*q):
    return frozenset(q)


# ---------------------------------------------------------------- types
class Type:
    cv = NOCV


@dataclass(frozen=True)
class Prim(Type):
    name: str
    cv: frozenset = NOCV

    def __repr__(self):
        return " ".join(sorted(self.cv)) + (" " if self.cv else "") + self.name


@dataclass(frozen=True)
class Ptr(Type):
    to: object
    cv: frozenset = NOCV

    def __repr__(self):
        return f"{self.to!r}*" + ("".join(" " + q for q in sorted(self.cv)))


@dataclass(frozen=True)
class LRef(Type):
    to: object

    def __repr__(self):
        return f"{self.to!r}&"


@dataclass(frozen=True)
class RRef(Type):
    to: object

    def __repr__(self):
        return f"{self.to!r}&&"


@dataclass(frozen=True)
class Arr(Type):
    elem: object
    bound: object = None  # int, CParam, ValExpr, or None (unknown bound)

    @property
    def cv(self):
        return self.elem.cv

    def __repr__(self):
        return f"{self.elem!r}[{'' if self.bound is None else self.bound!r}]"


@dataclass(frozen=True)
class Func(Type):
    ret: object
    params: tuple = ()
    variadic: bool = False
    noexcept: object = False  # bool, or CParam

    def __repr__(self):
        ps = ", ".join(repr(p) for p in self.params) + (", ..." if self.variadic else "")
        ne = "" if self.noexcept is False else f" noexcept({self.noexcept!r})"
        return f"{self.ret!r}({ps}){ne}"


@dataclass(frozen=True)
class MemPtr(Type):
    cls: object
    to: object
    cv: frozenset = NOCV

    def __repr__(self):
        return f"{self.to!r} {self.cls!r}::*"


@dataclass(frozen=True)
class Cls(Type):
    """A class. name is a str, or a TTParam. args is () for a plain class."""
    name: object
    args: tuple = ()
    cv: frozenset = NOCV
    is_template: bool = False  # a specialization of a class template

    def __repr__(self):
        q = " ".join(sorted(self.cv)) + (" " if self.cv else "")
        if self.is_template or self.args:
            return f"{q}{self.name!r}<{', '.join(repr(a) for a in self.args)}>".replace("'", "")
        return f"{q}{self.name}"


@dataclass(frozen=True)
class TParam(Type):
    name: str
    cv: frozenset = NOCV

    def __repr__(self):
        return " ".join(sorted(self.cv)) + (" " if self.cv else "") + self.name


@dataclass(frozen=True)
class Expand:
    """A pack expansion in a parameter list or a template argument list."""
    pattern: object

    def __repr__(self):
        return f"{self.pattern!r}..."


@dataclass(frozen=True)
class Qualified(Type):
    """typename Scope::member. A non-deduced position (DED-13)."""
    scope: object
    member: str
    cv: frozenset = NOCV

    def __repr__(self):
        return f"{self.scope!r}::{self.member}"


@dataclass(frozen=True)
class Decltype(Type):
    """decltype(expr). fn(bindings, env) gives a Type, or raises SubstFail / Hard."""
    fn: Callable
    text: str = "decltype(...)"
    cv: frozenset = NOCV

    def __repr__(self):
        return self.text


@dataclass(frozen=True)
class AliasT(Type):
    """An alias template specialization; env.aliases[name](args) gives the aliased type."""
    name: str
    args: tuple

    def __repr__(self):
        return f"{self.name}<{', '.join(map(repr, self.args))}>"


# ---------------------------------------------------------------- values
@dataclass(frozen=True)
class Val:
    value: object
    type: object

    def __repr__(self):
        return repr(self.value)


@dataclass(frozen=True)
class CParam:
    name: str

    def __repr__(self):
        return self.name


@dataclass(frozen=True)
class ValExpr:
    """An expression of constant parameters, e.g. N + 1. fn(bindings) gives an int."""
    fn: Callable
    text: str = "expr"

    def __repr__(self):
        return self.text


@dataclass(frozen=True)
class TmplName:
    name: str

    def __repr__(self):
        return self.name


@dataclass(frozen=True)
class TTParam:
    name: str

    def __repr__(self):
        return self.name


SIZE_T = Prim("unsigned long")  # notes 26
BOOL = Prim("bool")
INT = Prim("int")
VOID = Prim("void")


# ---------------------------------------------------------------- declarations
@dataclass
class TP:
    """A template parameter. kind: 'type', 'const', 'template'.
    ctype: the type of a constant parameter (a Type, a TParam, or Prim('auto') / Ptr(Prim('auto')))."""
    name: str
    kind: str = "type"
    pack: bool = False
    default: object = None
    ctype: object = None


@dataclass
class FP:
    """A function parameter. pack: a function parameter pack, type is its pattern."""
    type: object
    default: bool = False
    pack: bool = False


@dataclass
class Template:
    name: str
    tparams: list
    params: list
    ret: object = VOID
    trailing: bool = False       # trailing result type: substituted after the parameters
    variadic: bool = False       # C ellipsis
    deduced_ret: bool = False    # result type 'auto', from the body
    body_error: bool = False     # the body has an error for these arguments
    body_error_if: Callable = None  # fn(bindings) -> True if the body has an error
    noexcept_error: Callable = None  # fn(bindings) -> True: DED-27
    explicit_cond: Callable = None   # fn(bindings): raises SubstFail for a bad conditional explicit
    constraint: Callable = None      # fn(bindings) -> bool  (notes 22)
    tparam_checks: list = field(default_factory=list)  # extra types in template parameter declarations


@dataclass
class Plain:
    """A function that is not a template."""
    name: str
    type: Func
    defaults: int = 0  # number of trailing parameters with default arguments


@dataclass
class ClassDef:
    bases: list = field(default_factory=list)          # Types (Cls), may be TParam-free
    members: dict = field(default_factory=dict)         # name -> (Type, access)
    body_error: bool = False
    complete: bool = True
    converts_from: list = field(default_factory=list)   # types a non-explicit constructor takes
    virtual_bases: list = field(default_factory=list)


@dataclass
class Env:
    classes: dict = field(default_factory=dict)          # plain name -> ClassDef
    class_templates: dict = field(default_factory=dict)  # name -> fn(args) -> ClassDef
    ctemplate_params: dict = field(default_factory=dict)  # name -> list of TP (for DED-12)
    aliases: dict = field(default_factory=dict)          # name -> fn(args) -> Type
    ctemplate_constraints: dict = field(default_factory=dict)  # name -> fn(args) -> bool

    def classdef(self, c, hard_ok=True):
        if c.is_template or c.args:
            name = c.name.name if isinstance(c.name, TTParam) else c.name
            fn = self.class_templates.get(name)
            if fn is None:
                return ClassDef()
            d = fn(c.args)
        else:
            d = self.classes.get(c.name, ClassDef())
        if d.body_error:
            raise Hard("DED-29", f"the body of {c!r} has an error, and deduction instantiates it")
        return d


# ---------------------------------------------------------------- arguments
@dataclass
class Arg:
    type: object
    cat: str = "prvalue"   # lvalue, xvalue, prvalue
    null: bool = False     # a null pointer constant (0, nullptr)


@dataclass
class Braced:
    elems: list


@dataclass
class OverloadArg:
    funcs: list            # Func types of the plain functions in the set
    has_template: bool = False


# ---------------------------------------------------------------- results
@dataclass
class Deduced:
    args: dict
    ptypes: list = field(default_factory=list)
    ret: object = None
    viable: bool = True
    why: str = ""
    ranks: list = field(default_factory=list)

    kind = "deduced"


@dataclass
class Failure:
    rule: str
    why: str
    kind = "failure"


@dataclass
class HardError:
    rule: str
    why: str
    kind = "hard"


class SubstFail(Exception):
    def __init__(self, rule, why):
        super().__init__(why)
        self.rule, self.why = rule, why


class Hard(Exception):
    def __init__(self, rule, why):
        super().__init__(why)
        self.rule, self.why = rule, why


class PairFail(Exception):
    def __init__(self, rule, why):
        super().__init__(why)
        self.rule, self.why = rule, why


# ---------------------------------------------------------------- helpers
def with_cv(t, q):
    q = frozenset(q)
    if isinstance(t, (LRef, RRef, Func)):
        return t  # notes 14
    if isinstance(t, Arr):
        return Arr(with_cv(t.elem, q), t.bound)
    if isinstance(t, (Prim, Ptr, MemPtr, Cls, TParam, Qualified)):
        return type(t)(**{**t.__dict__, "cv": q})
    return t


def add_cv(t, q):
    return with_cv(t, cv_of(t) | frozenset(q))


def cv_of(t):
    return getattr(t, "cv", NOCV) or NOCV


def strip_cv(t):
    return with_cv(t, NOCV)


INTEGRAL = ["bool", "char", "signed char", "unsigned char", "char8_t", "char16_t", "char32_t", "wchar_t",
            "short", "unsigned short", "int", "unsigned int", "long", "unsigned long",
            "long long", "unsigned long long"]
FLOATING = ["float", "double", "long double"]
PROMOTE_TO_INT = {"bool", "char", "signed char", "unsigned char", "char8_t", "char16_t", "wchar_t",
                  "short", "unsigned short"}
RANGE = {"bool": (0, 1), "char": (-128, 127), "signed char": (-128, 127), "unsigned char": (0, 255),
         "short": (-2**15, 2**15 - 1), "unsigned short": (0, 2**16 - 1), "int": (-2**31, 2**31 - 1),
         "unsigned int": (0, 2**32 - 1), "long": (-2**63, 2**63 - 1), "unsigned long": (0, 2**64 - 1),
         "long long": (-2**63, 2**63 - 1), "unsigned long long": (0, 2**64 - 1),
         "char8_t": (0, 255), "char16_t": (0, 2**16 - 1), "char32_t": (0, 2**32 - 1),
         "wchar_t": (-2**31, 2**31 - 1)}


def arith(t):
    return isinstance(t, Prim) and (t.name in INTEGRAL or t.name in FLOATING)


def is_class(t):
    return isinstance(t, Cls)


def tparams_in(t, acc=None):
    """Names of template parameters that occur in t."""
    if acc is None:
        acc = set()
    if isinstance(t, TParam):
        acc.add(t.name)
    elif isinstance(t, CParam):
        acc.add(t.name)
    elif isinstance(t, TTParam):
        acc.add(t.name)
    elif isinstance(t, ValExpr):
        acc.update(getattr(t.fn, "names", ()))
    elif isinstance(t, Decltype):
        acc.update(getattr(t.fn, "names", ()))
    elif isinstance(t, (Ptr, LRef, RRef)):
        tparams_in(t.to, acc)
    elif isinstance(t, Arr):
        tparams_in(t.elem, acc)
        tparams_in(t.bound, acc)
    elif isinstance(t, Func):
        tparams_in(t.ret, acc)
        for p in t.params:
            tparams_in(p, acc)
        tparams_in(t.noexcept, acc)
    elif isinstance(t, MemPtr):
        tparams_in(t.cls, acc)
        tparams_in(t.to, acc)
    elif isinstance(t, Cls):
        tparams_in(t.name, acc)
        for a in t.args:
            tparams_in(a, acc)
    elif isinstance(t, Expand):
        tparams_in(t.pattern, acc)
    elif isinstance(t, Qualified):
        tparams_in(t.scope, acc)
    elif isinstance(t, AliasT):
        for a in t.args:
            tparams_in(a, acc)
    elif isinstance(t, (tuple, list)):
        for a in t:
            tparams_in(a, acc)
    return acc


# ---------------------------------------------------------------- substitution
def subst(t, b, env, fixed_only=False):
    """Replace template parameters by their values. Raises SubstFail for an invalid
    result (DED-24), Hard for an error outside the function type (DED-29).
    With fixed_only, parameters without a value stay."""
    def look(name):
        if name in b:
            return b[name]
        if fixed_only:
            return None
        raise SubstFail("DED-23", f"{name} has no value")

    if isinstance(t, TParam):
        v = look(t.name)
        if v is None:
            return t
        if isinstance(v, tuple):
            raise SubstFail("PACK-8", f"pack {t.name} used outside an expansion")
        return add_cv(v, t.cv)
    if isinstance(t, Prim):
        return t
    if isinstance(t, Ptr):
        s = subst(t.to, b, env, fixed_only)
        if isinstance(s, (LRef, RRef)):
            raise SubstFail("DED-24", f"a pointer to a reference ({s!r})")
        return Ptr(s, t.cv)
    if isinstance(t, (LRef, RRef)):
        s = subst(t.to, b, env, fixed_only)
        if isinstance(s, Prim) and s.name == "void":
            raise SubstFail("DED-24", "a reference to void")
        if isinstance(s, LRef):
            return LRef(s.to)
        if isinstance(s, RRef):
            return LRef(s.to) if isinstance(t, LRef) else RRef(s.to)
        return type(t)(s)
    if isinstance(t, Arr):
        e = subst(t.elem, b, env, fixed_only)
        if isinstance(e, (LRef, RRef)):
            raise SubstFail("DED-24", "an array of references")
        if isinstance(e, Func):
            raise SubstFail("DED-24", "an array of functions")  # notes 15
        if isinstance(e, Prim) and e.name == "void":
            raise SubstFail("DED-24", "an array of void")  # notes 15
        bound = subst_val(t.bound, b, env, fixed_only)
        if isinstance(bound, Val):
            if bound.value <= 0:
                raise SubstFail("DED-24", f"an array bound of {bound.value}")
            bound = bound.value
        elif isinstance(bound, int) and bound <= 0:
            raise SubstFail("DED-24", f"an array bound of {bound}")
        return Arr(e, bound)
    if isinstance(t, Func):
        r = subst(t.ret, b, env, fixed_only)
        if isinstance(r, (Arr, Func)):
            raise SubstFail("DED-24", "a function that returns an array or a function")
        ps = subst_list(t.params, b, env, fixed_only, in_params=True)
        ne = subst_val(t.noexcept, b, env, fixed_only)
        if isinstance(ne, Val):
            ne = bool(ne.value)
        return Func(r, tuple(adjust_param(p) for p in ps), t.variadic, ne)
    if isinstance(t, MemPtr):
        c = subst(t.cls, b, env, fixed_only)
        if not isinstance(c, (Cls, TParam)):
            raise SubstFail("DED-24", f"a pointer to member of {c!r}, which is not a class")
        to = subst(t.to, b, env, fixed_only)
        if isinstance(to, (LRef, RRef)):
            raise SubstFail("DED-24", "a pointer to member of reference type")
        return MemPtr(c, to, t.cv)
    if isinstance(t, Cls):
        name = t.name
        if isinstance(name, TTParam):
            v = look(name.name)
            name = name if v is None else v.name
        args = subst_list(t.args, b, env, fixed_only)
        out = Cls(name, tuple(args), t.cv, t.is_template or bool(args))
        if not fixed_only and not tparams_in(out):
            chk = env.ctemplate_constraints.get(name if isinstance(name, str) else "")
            if chk is not None and not chk(out.args):
                raise SubstFail("DED-24", f"{out!r} does not satisfy the constraints of its template (CNS-13)")
        return out
    if isinstance(t, Qualified):
        s = subst(t.scope, b, env, fixed_only)
        if tparams_in(s):
            return Qualified(s, t.member, t.cv)
        if not isinstance(s, Cls):
            raise SubstFail("DED-24", f"{s!r}::{t.member}, and {s!r} is not a class")
        d = env.classdef(s)  # notes 4: a qualified name instantiates the class
        if not d.complete:
            raise SubstFail("DED-24", f"{s!r} is incomplete")  # notes 5
        if t.member not in d.members:
            raise SubstFail("DED-24", f"{s!r} has no member {t.member}")
        mt, access = d.members[t.member]
        if access != "public":
            raise SubstFail("DED-26", f"{s!r}::{t.member} is {access}")
        if mt is None:
            raise SubstFail("DED-24", f"{s!r}::{t.member} is not a type")
        return add_cv(mt, t.cv)
    if isinstance(t, Decltype):
        if fixed_only and not set(getattr(t.fn, "names", ())) <= set(b):
            return t
        return add_cv(t.fn(b, env), t.cv)
    if isinstance(t, AliasT):
        args = subst_list(t.args, b, env, fixed_only)
        return subst(env.aliases[t.name](tuple(args)), b, env, fixed_only)
    if isinstance(t, (Val, TmplName, CParam, ValExpr, TTParam)) or t is None or isinstance(t, (int, bool)):
        return subst_val(t, b, env, fixed_only)
    raise TypeError(f"cannot substitute {t!r}")


def subst_val(v, b, env, fixed_only=False):
    if isinstance(v, CParam):
        if v.name in b:
            return b[v.name]
        if fixed_only:
            return v
        raise SubstFail("DED-23", f"{v.name} has no value")
    if isinstance(v, TTParam):
        if v.name in b:
            return b[v.name]
        if fixed_only:
            return v
        raise SubstFail("DED-23", f"{v.name} has no value")
    if isinstance(v, ValExpr):
        names = getattr(v.fn, "names", ())
        if fixed_only and not set(names) <= set(b):
            return v
        r = v.fn(b)
        return r if isinstance(r, Val) else Val(r, INT)
    if isinstance(v, (Val, TmplName, bool, int)) or v is None:
        return v
    return subst(v, b, env, fixed_only)


def subst_list(items, b, env, fixed_only, in_params=False):
    out = []
    for it in items:
        if isinstance(it, Expand):
            packs = [n for n in tparams_in(it.pattern) if isinstance(b.get(n), tuple)]
            unknown = [n for n in tparams_in(it.pattern) if n not in b]
            if fixed_only and unknown:
                out.append(it)
                continue
            if not packs:
                raise SubstFail("PACK-8", f"no pack in {it!r}")
            lens = {len(b[n]) for n in packs}
            if len(lens) > 1:
                raise SubstFail("PACK-27", f"packs of different lengths in {it!r}")
            n = lens.pop()
            for i in range(n):
                bi = dict(b)
                for p in packs:
                    bi[p] = b[p][i]
                e = subst(it.pattern, bi, env, fixed_only)
                if in_params and isinstance(e, Prim) and e.name == "void" and not cv_of(e):
                    raise SubstFail("PACK-27", "an expansion gives a parameter of type void (PACK-28)")
                out.append(e)
        else:
            out.append(subst(it, b, env, fixed_only))
    return out


def adjust_param(p):
    """DCL-8: an array parameter is a pointer, a function parameter is a pointer, top-level cv goes."""
    if isinstance(p, Arr):
        return Ptr(p.elem)
    if isinstance(p, Func):
        return Ptr(p)
    if isinstance(p, Expand):
        return p
    return strip_cv(p)


# ---------------------------------------------------------------- matching (DED-10 to DED-15, DED-36, DED-37)
def bind(b, name, val):
    if name in b and b[name] != val:
        raise PairFail("DED-21", f"{name} is {b[name]!r} and {val!r}")
    b[name] = val


def is_nondeduced(p):
    return isinstance(p, (Qualified, Decltype, ValExpr))


def unify(P, A, b, env, loose):
    """Match P (with parameters) against A. Raises PairFail on mismatch.
    loose: allow P to have more cv than A at this level (DED-14)."""
    if isinstance(P, AliasT):
        P = env.aliases[P.name](P.args)
    if is_nondeduced(P):
        return
    if isinstance(P, TParam):
        cp, ca = cv_of(P), cv_of(A)
        if not cp <= ca and not loose:
            raise PairFail("DED-10", f"{P!r} against {A!r}")
        if cp and isinstance(A, (LRef, RRef, Func)):
            raise PairFail("DED-10", f"{P!r} against {A!r}")
        bind(b, P.name, with_cv(A, ca - cp) if not isinstance(A, (LRef, RRef, Func)) else A)
        return
    cp, ca = cv_of(P), cv_of(A)
    if not isinstance(P, Arr):
        if cp != ca and not (loose and cp >= ca):
            raise PairFail("DED-10", f"{P!r} against {A!r}: qualifiers differ")
    if isinstance(P, Prim):
        if not isinstance(A, Prim) or P.name != A.name:
            raise PairFail("DED-10", f"{P!r} against {A!r}")
        return
    if isinstance(P, Ptr):
        if not isinstance(A, Ptr):
            raise PairFail("DED-10", f"{P!r} against {A!r}")
        unify(P.to, A.to, b, env, loose)
        return
    if isinstance(P, (LRef, RRef)):
        if type(P) is not type(A):
            raise PairFail("DED-10", f"{P!r} against {A!r}")
        unify(P.to, A.to, b, env, False)
        return
    if isinstance(P, Arr):
        if not isinstance(A, Arr):
            raise PairFail("DED-10", f"{P!r} against {A!r}")
        unify(P.elem, A.elem, b, env, loose)
        unify_bound(P.bound, A.bound, b, env)
        return
    if isinstance(P, Func):
        if not isinstance(A, Func) or P.variadic != A.variadic:
            raise PairFail("DED-10", f"{P!r} against {A!r}")
        unify(P.ret, A.ret, b, env, False)
        unify_list(P.params, A.params, b, env)
        if isinstance(P.noexcept, CParam):
            bind(b, P.noexcept.name, Val(bool(A.noexcept), BOOL))
        elif bool(P.noexcept) != bool(A.noexcept):
            if not (loose and not P.noexcept and A.noexcept):
                raise PairFail("DED-10", "noexcept differs")
        return
    if isinstance(P, MemPtr):
        if not isinstance(A, MemPtr):
            raise PairFail("DED-10", f"{P!r} against {A!r}")
        unify(P.cls, A.cls, b, env, False)
        unify(P.to, A.to, b, env, loose)
        return
    if isinstance(P, Cls):
        if not isinstance(A, Cls):
            raise PairFail("DED-10", f"{P!r} against {A!r}")
        if isinstance(P.name, TTParam):
            if not (A.is_template or A.args):
                raise PairFail("DED-10", f"{A!r} is not a specialization")
            bind(b, P.name.name, TmplName(A.name))
        elif P.name != A.name:
            raise PairFail("DED-10", f"{P!r} against {A!r}")
        unify_targs(P.args, A.args, A.name, b, env)
        return
    raise PairFail("DED-10", f"cannot match {P!r}")


def unify_bound(pb, ab, b, env):
    if isinstance(pb, CParam):
        if ab is None:
            raise PairFail("DED-10", "unknown bound")
        bind(b, pb.name, Val(ab, SIZE_T))
        b.setdefault("__bound__", set()).add(pb.name)
        return
    if isinstance(pb, ValExpr) or pb is None and ab is None:
        return
    if pb != ab:
        raise PairFail("DED-10", f"bound {pb!r} against {ab!r}")


def unify_list(ps, as_, b, env):
    """Function parameter lists in a function type (DED-37, PACK-12)."""
    exp_at = [i for i, p in enumerate(ps) if isinstance(p, Expand)]
    if exp_at and exp_at[0] != len(ps) - 1:
        return  # PACK-12: non-deduced list
    if exp_at:
        head = ps[:-1]
        if len(as_) < len(head):
            raise PairFail("DED-10", "too few parameters")
        for p, a in zip(head, as_):
            unify(adjust_param(p), a, b, env, False)
        unify_pack(ps[-1].pattern, list(as_[len(head):]), b, env, lambda P, A, bb: unify(adjust_param(P), A, bb, env, False))
        return
    if len(ps) != len(as_):
        raise PairFail("DED-10", "parameter counts differ")
    for p, a in zip(ps, as_):
        unify(adjust_param(p), a, b, env, False)


def unify_targs(ps, as_, tname, b, env):
    exp_at = [i for i, p in enumerate(ps) if isinstance(p, Expand)]
    if exp_at and exp_at[0] != len(ps) - 1:
        return  # PACK-12
    tps = env.ctemplate_params.get(tname if isinstance(tname, str) else "", [])
    head = ps[:-1] if exp_at else ps
    if not exp_at and len(ps) != len(as_):
        raise PairFail("DED-10", "argument counts differ")
    if len(as_) < len(head):
        raise PairFail("DED-10", "too few arguments")
    for i, (p, a) in enumerate(zip(head, as_)):
        unify_targ(p, a, tps[i] if i < len(tps) else None, b, env)
    if exp_at:
        rest = list(as_[len(head):])
        unify_pack(ps[-1].pattern, rest, b, env,
                   lambda P, A, bb: unify_targ(P, A, tps[-1] if tps else None, bb, env))


def unify_targ(p, a, ctp, b, env):
    if isinstance(p, CParam):
        if not isinstance(a, Val):
            raise PairFail("DED-10", f"{p!r} against {a!r}")
        bind(b, p.name, a)
        b.setdefault("__ctype__", {})[p.name] = a.type
        return
    if isinstance(p, ValExpr):
        return
    if isinstance(p, Val):
        if not isinstance(a, Val) or a.value != p.value:
            raise PairFail("DED-10", f"{p!r} against {a!r}")
        return
    if isinstance(p, TTParam):
        if not isinstance(a, TmplName):
            raise PairFail("DED-10", f"{p!r} against {a!r}")
        bind(b, p.name, a)
        return
    if isinstance(p, TmplName):
        if p != a:
            raise PairFail("DED-10", f"{p!r} against {a!r}")
        return
    if isinstance(a, (Val, TmplName)):
        raise PairFail("DED-10", f"{p!r} against {a!r}")
    unify(p, a, b, env, False)


def unify_pack(pattern, items, b, env, one):
    """Each item pairs with the pattern; each pack in the pattern collects one element per item."""
    packs = [n for n in tparams_in(pattern) if b.get("__packs__", set()) and n in b["__packs__"]]
    allp = b.get("__packs__", set())
    names = [n for n in tparams_in(pattern) if n in allp]
    elems = {n: [] for n in names}
    for it in items:
        bb = {k: v for k, v in b.items() if k not in names}
        one(pattern, it, bb)
        for n in names:
            if n not in bb:
                raise PairFail("DED-10", f"no element for {n}")
            elems[n].append(bb[n])
        for k, v in bb.items():
            if k not in names and not k.startswith("__"):
                bind(b, k, v)
    for n in names:
        bind(b, n, tuple(elems[n]))


def ref_related_base(env, A, P):
    """Bases of class A (transitively) that are specializations, with derivation info."""
    out = []

    def walk(c, path):
        d = env.classdef(c)
        for base in d.bases:
            out.append((base, path + [c]))
            walk(base, path + [c])
    walk(strip_cv(A), [])
    return out


def match_pair(P, A, b, env, tmpl_names):
    """One pair (DED-10, DED-14, DED-15). A is the adjusted argument type, P the adjusted parameter."""
    trial = dict(b)
    try:
        unify(P, A, trial, env, False)
        b.clear(); b.update(trial)
        return
    except PairFail as e:
        first = e
    trial = dict(b)
    try:
        unify(P, A, trial, env, True)
        b.clear(); b.update(trial)
        return
    except PairFail:
        pass
    # DED-15: a base class that is a specialization
    core_p, core_a, wrap = P, A, None
    if isinstance(P, Ptr) and isinstance(A, Ptr):
        core_p, core_a, wrap = P.to, A.to, "ptr"
    if isinstance(core_p, Cls) and (core_p.is_template or core_p.args or isinstance(core_p.name, TTParam)) \
            and isinstance(core_a, Cls) and not (wrap == "ptr" and not cv_of(core_p) >= cv_of(core_a)) \
            and (wrap == "ptr" or cv_of(core_p) >= cv_of(core_a) or True):
        cands = []
        for base, path in ref_related_base(env, core_a, core_p):
            t = dict(b)
            try:
                unify(strip_cv(core_p), strip_cv(base), t, env, False)
            except PairFail:
                continue
            cands.append((base, t))
        if cands:
            # a base derived from another matching base wins over that base
            def derives(x, y):
                return any(strip_cv(bb) == strip_cv(y) for bb, _ in ref_related_base(env, x, None))
            keep = [c for c in cands if not any(derives(o[0], c[0]) for o in cands if o is not c)]
            vals = {tuple(sorted((k, repr(v)) for k, v in t.items() if not k.startswith("__"))) for _, t in keep}
            if len(vals) > 1:
                raise PairFail("DED-15", "two bases match with different values")
            b.clear(); b.update(keep[0][1])
            return
    raise first


def is_forwarding(P, tmpl):
    if not isinstance(P, RRef) or not isinstance(P.to, TParam) or P.to.cv:
        return False
    return any(tp.name == P.to.name and tp.kind == "type" for tp in tmpl.tparams)


def arg_type_for_match(arg):
    t = arg.type
    if arg.cat == "prvalue" and not is_class(t) and not isinstance(t, Arr):
        t = strip_cv(t)  # notes 1
    return t


def deduce_one(P, arg, b, env, tmpl):
    """Steps 2 and 3 for one pair. P is the declared parameter type (pattern for a pack)."""
    if isinstance(arg, Braced):
        return deduce_braced(P, arg, b, env, tmpl)
    if isinstance(arg, OverloadArg):
        return deduce_overload(P, arg, b, env, tmpl)
    A = arg_type_for_match(arg)
    P = strip_cv(P) if not isinstance(P, (LRef, RRef)) else P  # DED-7
    if isinstance(P, (LRef, RRef)):
        if is_forwarding(P, tmpl) and arg.cat == "lvalue":  # DED-8
            bind(b, P.to.name, LRef(A))
            return
        match_pair(P.to, A, b, env, tmpl)
        return
    # DED-6
    if isinstance(A, Arr):
        A = Ptr(A.elem)
    elif isinstance(A, Func):
        A = Ptr(A)
    else:
        A = strip_cv(A)
    P = adjust_param(P)
    match_pair(P, A, b, env, tmpl)


def init_list_elem(P):
    q = P.to if isinstance(P, (LRef, RRef)) else P
    q = strip_cv(q)
    if isinstance(q, Cls) and q.name == "std::initializer_list" and len(q.args) == 1:
        return q.args[0]
    return None


def deduce_braced(P, arg, b, env, tmpl):
    e = init_list_elem(P)
    if e is not None:  # DED-16
        for el in arg.elems:
            if isinstance(el, Braced):
                continue  # notes 20
            if isinstance(el, OverloadArg):
                deduce_overload(e, el, b, env, tmpl)
                continue
            deduce_one(e, el, b, env, tmpl)
        return
    if isinstance(P, (LRef, RRef)) and isinstance(P.to, Arr):  # DED-17
        arr = P.to
        if isinstance(arr.bound, CParam):
            bind(b, arr.bound.name, Val(len(arg.elems), SIZE_T))
            b.setdefault("__bound__", set()).add(arr.bound.name)
        elif isinstance(arr.bound, int) and arr.bound < len(arg.elems):
            raise PairFail("DED-17", "more elements than the bound")
        for el in arg.elems:
            if isinstance(el, Braced):
                deduce_braced(LRef(arr.elem), el, b, env, tmpl)
            else:
                deduce_one(strip_cv(arr.elem), el, b, env, tmpl)
        return
    return  # DED-18


def deduce_overload(P, arg, b, env, tmpl):
    """DED-19."""
    if arg.has_template:
        return
    hits = []
    for f in arg.funcs:
        t = dict(b)
        try:
            deduce_one(P, Arg(f, "lvalue"), t, env, tmpl)
        except PairFail:
            continue
        hits.append(t)
    if len(hits) == 1:
        b.clear(); b.update(hits[0])


# ---------------------------------------------------------------- conversions (notes 2)
EXACT, PROMO, CONVR, USER, ELLIPSIS = 0, 1, 2, 3, 4


def derived_from(env, d, base):
    if not isinstance(d, Cls) or not isinstance(base, Cls):
        return False
    try:
        bs = ref_related_base(env, d, None)
    except Hard:
        raise
    return any(strip_cv(x) == strip_cv(base) for x, _ in bs)


def base_count(env, d, base):
    return sum(1 for x, _ in ref_related_base(env, d, None) if strip_cv(x) == strip_cv(base))


def qual_ok(A, P):
    """A pointer A converts to P by a qualification adjustment (CONV-9 last bullet), or drops noexcept."""
    levels_a, levels_p = [], []
    a, p = A, P
    while isinstance(a, (Ptr, MemPtr)) and isinstance(p, type(a)):
        if isinstance(a, MemPtr) and strip_cv(a.cls) != strip_cv(p.cls):
            return False
        levels_a.append(cv_of(a.to)); levels_p.append(cv_of(p.to))
        a, p = a.to, p.to
    if not levels_a:
        return False
    if strip_cv(a) != strip_cv(p):
        if isinstance(a, Func) and isinstance(p, Func) and a.noexcept and not p.noexcept and \
                Func(a.ret, a.params, a.variadic, False) == p:
            pass
        else:
            return False
    # the inner levels: a level that adds requires const on each level above, except the outermost
    for i in range(len(levels_a)):
        if levels_a[i] != levels_p[i]:
            for j in range(0, i):
                if C not in levels_p[j]:
                    return False
    return True


def ics(arg, P, env):
    """A rank, or None when the argument has no implicit conversion sequence."""
    if isinstance(arg, Braced):
        return ics_braced(arg, P, env)
    if isinstance(arg, OverloadArg):
        q = P.to if isinstance(P, (LRef, RRef)) else strip_cv(P)
        q = q.to if isinstance(q, Ptr) else q
        return EXACT if any(f == q or (f.noexcept and Func(f.ret, f.params, f.variadic, False) == q) for f in arg.funcs) else None
    A, cat = arg.type, arg.cat
    if isinstance(P, (LRef, RRef)):
        R = P.to
        A0 = A
        related = strip_cv(R) == strip_cv(A0) or derived_from(env, strip_cv(A0), strip_cv(R))
        compat = related and cv_of(R) >= cv_of(A0)
        if isinstance(P, LRef) and not (C in cv_of(R) and V not in cv_of(R)):
            if cat == "lvalue" and compat:
                if strip_cv(R) != strip_cv(A0):
                    return CONVR if base_count(env, A0, R) == 1 else None
                return EXACT
            return None
        if isinstance(P, RRef) and cat == "lvalue" and related:
            return None
        if compat:
            if strip_cv(R) != strip_cv(A0):
                return CONVR if base_count(env, A0, R) == 1 else None
            return EXACT
        if related and not compat and (cat != "prvalue" or is_class(A0)):
            return None
        r = ics(Arg(A0, "prvalue", arg.null), strip_cv(R), env)
        if r is None:
            return None
        if isinstance(P, RRef) and cat == "lvalue" and r == EXACT:
            return None
        return r
    P = strip_cv(P)
    if isinstance(A, Arr):
        A = Ptr(A.elem)
    elif isinstance(A, Func):
        A = Ptr(A)
    A = strip_cv(A) if not is_class(A) else A
    if strip_cv(A) == P:
        return EXACT
    if arith(A) and arith(P):
        if (A.name in PROMOTE_TO_INT and P.name == "int") or (A.name == "float" and P.name == "double"):
            return PROMO
        return CONVR
    if isinstance(P, Ptr):
        if arg.null:
            return CONVR
        if isinstance(A, Prim) and A.name == "decltype(nullptr)":
            return CONVR
        if isinstance(A, Ptr):
            if qual_ok(A, P):
                return EXACT
            if isinstance(P.to, Prim) and P.to.name == "void" and not isinstance(A.to, Func) and cv_of(P.to) >= cv_of(A.to):
                return CONVR
            if is_class(A.to) and is_class(P.to) and cv_of(P.to) >= cv_of(A.to) and derived_from(env, A.to, P.to):
                return CONVR if base_count(env, A.to, P.to) == 1 else None
        return None
    if isinstance(P, MemPtr):
        if arg.null:
            return CONVR
        if isinstance(A, MemPtr) and qual_ok(A, P):
            return EXACT
        return None
    if isinstance(P, Prim) and P.name == "bool" and isinstance(A, (Ptr, MemPtr)):
        return CONVR
    if is_class(P):
        if is_class(A) and strip_cv(A) == P:
            return EXACT
        if is_class(A) and derived_from(env, A, P):
            return CONVR if base_count(env, A, P) == 1 else None
        d = env.classdef(P) if not tparams_in(P) else ClassDef()
        for src in d.converts_from:
            if src == strip_cv(A) or (arith(src) and arith(A)) or ics(Arg(A, cat, arg.null), src, env) is not None and not is_class(src):
                return USER
        return None
    return None


def ics_braced(arg, P, env):
    Q = P.to if isinstance(P, (LRef, RRef)) else strip_cv(P)
    if isinstance(P, LRef) and not (C in cv_of(P.to)):
        return None
    e = init_list_elem(P)
    if e is not None:
        worst = EXACT
        for el in arg.elems:
            r = ics(el, e, env) if not isinstance(el, Braced) else ics_braced(el, e, env)
            if r is None or (not isinstance(el, Braced) and narrows(el, e)):
                return None
            worst = max(worst, r)
        return worst
    if isinstance(Q, Arr):
        if isinstance(Q.bound, int) and Q.bound < len(arg.elems):
            return None
        worst = EXACT
        for el in arg.elems:
            r = ics(el, strip_cv(Q.elem), env) if not isinstance(el, Braced) else ics_braced(el, LRef(add_cv(Q.elem, {C})), env)
            if r is None:
                return None
            worst = max(worst, r)
        return worst
    if not is_class(Q):
        if len(arg.elems) == 0:
            return EXACT
        if len(arg.elems) == 1 and not isinstance(arg.elems[0], Braced):
            if narrows(arg.elems[0], Q):
                return None
            return ics(arg.elems[0], Q, env)
        return None
    if len(arg.elems) == 1 and isinstance(arg.elems[0], Arg):
        return ics(arg.elems[0], Q, env)
    return USER


def narrows(el, P):
    if not isinstance(el, Arg):
        return False
    A = strip_cv(el.type)
    if not (arith(A) and arith(P)):
        return False
    if A.name in FLOATING and P.name in INTEGRAL:
        return True
    if A.name in FLOATING and P.name in FLOATING:
        return FLOATING.index(P.name) < FLOATING.index(A.name)
    if A.name in INTEGRAL and P.name in FLOATING:
        return True
    if A.name in INTEGRAL and P.name in INTEGRAL:
        la, ha = RANGE[A.name]
        lp, hp = RANGE[P.name]
        return la < lp or ha > hp
    return False


# ---------------------------------------------------------------- deduction in a call
def fit_explicit(tmpl, explicit, env):
    """DED-2, DED-5."""
    b = {}
    tps = tmpl.tparams
    i = 0
    for e in explicit:
        if i >= len(tps):
            raise PairFail("DED-5", "more explicit arguments than template parameters")
        tp = tps[i]
        ok = (tp.kind == "type" and isinstance(e, Type)) or \
             (tp.kind == "const" and isinstance(e, Val)) or \
             (tp.kind == "template" and isinstance(e, TmplName))
        if not ok:
            raise PairFail("DED-5", f"explicit argument {e!r} does not fit {tp.name}")
        if tp.kind == "const":
            e = convert_const(e, tp, b, env)
        if tp.pack:
            b[tp.name] = b.get(tp.name, ()) + (e,)
        else:
            b[tp.name] = e
            i += 1
    return b


def convert_const(v, tp, b, env):
    ct = tp.ctype
    if ct is None or isinstance(ct, Prim) and ct.name == "auto":
        return v
    ct = subst(ct, b, env, fixed_only=True)
    if isinstance(ct, Prim) and ct.name in RANGE and isinstance(v.value, (int, bool)) and isinstance(v.type, Prim) and v.type.name in RANGE:
        lo, hi = RANGE[ct.name]
        if not lo <= int(v.value) <= hi:
            raise PairFail("DED-5", f"{v.value} narrows to {ct!r} (TARG-13)")
        return Val(int(v.value) if ct.name != "bool" else bool(v.value), ct)
    return v


def deduce(tmpl, args, explicit=(), env=None):
    env = env or Env()
    try:
        return _deduce(tmpl, args, explicit, env)
    except Hard as h:
        return HardError(h.rule, h.why)


def _deduce(tmpl, args, explicit, env):
    # step 1
    try:
        b = fit_explicit(tmpl, explicit, env)
    except PairFail as e:
        return Failure(e.rule, e.why)
    except SubstFail as e:
        return Failure("DED-5", e.why)
    packs = {tp.name for tp in tmpl.tparams if tp.pack}
    explicit_names = set(b)
    explicit_pack_len = {n: len(b[n]) for n in packs if n in b}
    b["__packs__"] = packs

    # step 2: pairs
    pairs = []  # (declared P, arg, takes_part, fixed_bindings)
    ai = 0
    nparams = len(tmpl.params)
    for pi, fp in enumerate(tmpl.params):
        last = pi == nparams - 1
        if fp.pack:
            pnames = [n for n in tparams_in(fp.type) if n in packs]
            k = min((explicit_pack_len.get(n, 0) for n in pnames), default=0)
            if last:
                rest = args[ai:]
                ai = len(args)
            else:
                rest = args[ai:ai + k]  # DED-20, notes 11
                if len(rest) < k:
                    return Failure("OVL-22", "too few arguments for the explicit pack elements")
                ai += k
            for j, a in enumerate(rest):
                if j < k:
                    bj = {n: b[n][j] for n in pnames}
                    pairs.append(("fixed", fp.type, a, bj))
                elif last:
                    pairs.append(("pack", fp.type, a, None))
            if not last:
                for n in pnames:
                    if n not in b:
                        b[n] = ()
            continue
        if ai < len(args):
            a = args[ai]
            ai += 1
            names = tparams_in(fp.type)
            if not names or names <= explicit_names:
                pairs.append(("fixed", fp.type, a, {}))  # DED-3
            else:
                pairs.append(("deduce", fp.type, a, None))
        elif fp.default:
            continue  # DED-9
        else:
            return Failure("OVL-22", f"no argument for parameter {pi + 1}")
    if ai < len(args) and not tmpl.variadic:
        return Failure("OVL-22", "more arguments than parameters")
    extra = args[ai:]

    # step 3 and 4: match and combine
    pack_elems = {n: [] for n in packs}
    try:
        for kind, P, a, _ in pairs:
            if kind == "deduce":
                trial = dict(b)
                deduce_one(P, a, trial, env, tmpl)
                merge(b, trial)
            elif kind == "pack":
                pnames = [n for n in tparams_in(P) if n in packs]
                trial = {k: v for k, v in b.items() if k not in pnames}
                deduce_one(P, a, trial, env, tmpl)
                for n in pnames:
                    if n not in trial:
                        raise PairFail("DED-23", f"no element for {n}")
                    pack_elems[n].append(trial[n])
                merge(b, {k: v for k, v in trial.items() if k not in pnames})
    except PairFail as e:
        return Failure(e.rule, e.why)
    for n in packs:
        if pack_elems[n] or n in explicit_pack_len:
            new = tuple(b.get(n, ())) + tuple(pack_elems[n])
            b[n] = new
    # DED-22: a bound gives a type only if no other pair does
    weak = b.pop("__bound__", set())
    ctypes = b.pop("__ctype__", {})
    b.pop("__packs__", None)
    for tp in tmpl.tparams:
        if tp.kind == "const" and tp.name in b and isinstance(tp.ctype, TParam):
            u = tp.ctype.name
            vt = b[tp.name].type if isinstance(b[tp.name], Val) else None
            if u not in b and vt is not None:
                b[u] = vt
        if tp.kind == "const" and tp.name in b and tp.name in ctypes:  # DED-12
            ct = tp.ctype
            if ct is not None and not tparams_in(ct):
                if isinstance(ct, Prim) and ct.name == "auto":
                    b[tp.name] = Val(b[tp.name].value, ctypes[tp.name])
                elif isinstance(ct, Ptr) and isinstance(ct.to, Prim) and ct.to.name == "auto":
                    if not isinstance(ctypes[tp.name], Ptr):
                        return Failure("DED-12", f"{ctypes[tp.name]!r} does not have the form of {ct!r}")
                elif strip_cv(ct) != strip_cv(ctypes[tp.name]):
                    return Failure("DED-12", f"{tp.name} has type {ct!r}, the class template's parameter has {ctypes[tp.name]!r}")
        if tp.kind == "const" and tp.name in b and tp.name in weak and isinstance(tp.ctype, Prim) \
                and tp.ctype.name != "auto" and isinstance(b[tp.name], Val):
            b[tp.name] = Val(b[tp.name].value, tp.ctype)
        if tp.kind == "const" and tp.name in b and tp.name in weak and isinstance(tp.ctype, Prim) and tp.ctype.name == "auto":
            b[tp.name] = Val(b[tp.name].value, SIZE_T)

    # step 5a: defaults
    try:
        for tp in tmpl.tparams:
            if tp.name in b:
                continue
            if tp.pack:
                b[tp.name] = ()
            elif tp.default is not None:
                d = tp.default
                b[tp.name] = subst(d, b, env) if not isinstance(d, (Val, TmplName)) else d
            else:
                return Failure("DED-23", f"{tp.name} has no value")
        for t in tmpl.tparam_checks:
            subst(t, b, env)
        for tp in tmpl.tparams:
            if tp.kind == "const" and tp.ctype is not None and tparams_in(tp.ctype):
                subst(tp.ctype, b, env)
    except SubstFail as e:
        return Failure(e.rule if e.rule != "DED-23" else "DED-24", e.why)

    # step 5b: DED-28
    for kind, P, a, _ in pairs:
        if kind == "fixed" and not tparams_in(P):
            if ics(a, adjust_param(P) if not isinstance(P, (LRef, RRef)) else P, env) is None:
                return Failure("DED-28", f"argument has no conversion to {P!r}")

    # step 5c: the function type, in source order
    try:
        ptypes = []
        ret = None
        if not tmpl.trailing and not tmpl.deduced_ret:
            ret = subst(tmpl.ret, b, env)
            if isinstance(ret, (Arr, Func)):
                raise SubstFail("DED-24", "a function that returns an array or a function")
        for fp in tmpl.params:
            if fp.pack:
                ptypes.extend(subst_list([Expand(fp.type)], b, env, False, in_params=True))
            else:
                ptypes.append(subst(fp.type, b, env))
        if tmpl.trailing and not tmpl.deduced_ret:
            ret = subst(tmpl.ret, b, env)
            if isinstance(ret, (Arr, Func)):
                raise SubstFail("DED-24", "a function that returns an array or a function")
        if tmpl.explicit_cond:
            tmpl.explicit_cond(b)
    except SubstFail as e:
        return Failure(e.rule if e.rule != "DED-23" else "DED-24", e.why)
    ptypes = [adjust_param(p) for p in ptypes]
    out = {k: v for k, v in b.items() if not k.startswith("__")}
    if tmpl.constraint is not None and not tmpl.constraint(out):
        return Failure("CNS-11", "the constraints are not satisfied")

    # viability (OVL-22, OVL-23)
    used = [a for _, _, a, _ in pairs]
    ranks = []
    for a, p in zip(used, ptypes):
        r = ics(a, p, env)
        if r is None:
            return Deduced(out, ptypes, ret, False, f"no conversion from the argument to {p!r}")
        ranks.append(r)
    ranks.extend([ELLIPSIS] * len(extra))
    return Deduced(out, ptypes, ret, True, "", ranks)


def merge(b, trial):
    for k, v in trial.items():
        if k in ("__bound__",):
            b.setdefault(k, set()).update(v)
            continue
        if k == "__ctype__":
            b.setdefault(k, {}).update(v)
            continue
        if k == "__packs__":
            continue
        if k in b and b[k] != v:
            # DED-22: a bound value yields its type to another pair
            raise PairFail("DED-21", f"{k} is {b[k]!r} and {v!r}")
        b[k] = v


# ---------------------------------------------------------------- other contexts
def deduce_target(tmpl, target, env=None):
    """DED-30: match the template's function type with a target function type."""
    env = env or Env()
    b = {"__packs__": {tp.name for tp in tmpl.tparams if tp.pack}}
    ftype = Func(tmpl.ret, tuple(Expand(p.type) if p.pack else p.type for p in tmpl.params), tmpl.variadic)
    try:
        if len(ftype.params) == len(target.params):
            for p, a in zip(ftype.params, target.params):
                if is_forwarding(p, tmpl) and isinstance(a, LRef):
                    bind(b, p.to.name, a)
                else:
                    unify(adjust_param(p), a, b, env, False)
            if not tmpl.deduced_ret:
                unify(tmpl.ret, target.ret, b, env, False)
        else:
            unify_list(ftype.params, target.params, b, env)
            unify(tmpl.ret, target.ret, b, env, False)
    except PairFail as e:
        return Failure(e.rule, e.why)
    b.pop("__packs__", None)
    b.pop("__bound__", None)
    b.pop("__ctype__", None)
    for tp in tmpl.tparams:
        if tp.name not in b:
            if tp.pack:
                b[tp.name] = ()
            elif tp.default is not None:
                b[tp.name] = tp.default
            else:
                return Failure("DED-23", f"{tp.name} has no value")
    return Deduced(b)


# ---------------------------------------------------------------- the call
@dataclass
class Chosen:
    index: int
    result: object
    kind = "chosen"


@dataclass
class Reject:
    rule: str
    why: str
    kind = "reject"


def plain_ranks(f, args, env):
    ps = list(f.type.params)
    if len(args) > len(ps) and not f.type.variadic:
        return None
    if len(args) < len(ps) - f.defaults:
        return None
    ranks = []
    for a, p in zip(args, ps):
        r = ics(a, p, env)
        if r is None:
            return None
        ranks.append(r)
    ranks.extend([ELLIPSIS] * max(0, len(args) - len(ps)))
    return ranks


def at_least_as_specialized(t1, t2, env):
    """notes 16: a rough partial ordering, from DED-P1's line on invented types."""
    b = {tp.name: Cls(f"__U_{tp.name}") if tp.kind == "type" else Val(10007, INT) for tp in t1.tparams}
    for tp in t1.tparams:
        if tp.pack:
            b[tp.name] = (b[tp.name],)
    try:
        ps1 = [subst(p.type, b, env) if not p.pack else subst_list([Expand(p.type)], b, env, False)[0] for p in t1.params]
    except (SubstFail, Hard):
        return False
    args = []
    for p in ps1:
        if isinstance(p, LRef):
            args.append(Arg(p.to, "lvalue"))
        elif isinstance(p, RRef):
            args.append(Arg(p.to, "xvalue"))
        else:
            args.append(Arg(p, "lvalue"))
    t2c = Template(t2.name, t2.tparams, [FP(strip_ref_fp(p.type), p.default, p.pack) for p in t2.params], VOID)
    r = deduce(t2c, args[:len(t2.params)] if not any(p.pack for p in t2.params) else args, (), env)
    return isinstance(r, Deduced)


def strip_ref_fp(t):
    if isinstance(t, (LRef, RRef)):
        return LRef(strip_cv(t.to)) if False else t
    return t


def better(ri, rj):
    return all(a <= b for a, b in zip(ri, rj)) and any(a < b for a, b in zip(ri, rj))


def resolve(cands, args, explicit=(), env=None):
    """Candidates: Template and Plain. A Plain is a candidate only without explicit arguments (DED-4)."""
    env = env or Env()
    viable = []
    for i, c in enumerate(cands):
        if isinstance(c, Plain):
            if explicit:
                continue
            try:
                r = plain_ranks(c, args, env)
            except Hard as h:
                return Reject(h.rule, h.why)
            if r is not None:
                viable.append((i, c, r, None))
        else:
            d = deduce(c, args, explicit, env)
            if isinstance(d, HardError):
                return Reject(d.rule, d.why)
            if isinstance(d, Deduced) and d.viable:
                viable.append((i, c, d.ranks, d))
    if not viable:
        return Reject("DED-35" if any(isinstance(c, Template) for c in cands) else "OVL-35", "no viable function")
    best = []
    for v in viable:
        if all(v is w or beats(v, w, env) for w in viable):
            best.append(v)
    if len(best) != 1:
        return Reject("OVL-36", "ambiguous")
    i, c, r, d = best[0]
    if isinstance(c, Template):
        if c.body_error or (c.body_error_if and c.body_error_if(d.args)):
            return Reject("DED-29", "the body of the chosen specialization has an error")
        if c.noexcept_error and c.noexcept_error(d.args):
            return Reject("DED-27", "the noexcept operand of the chosen specialization has an error")
    return Chosen(i, d)


def beats(v, w, env):
    _, c1, r1, _ = v
    _, c2, r2, _ = w
    if any(a > b for a, b in zip(r1, r2)):
        return False
    if any(a < b for a, b in zip(r1, r2)):
        return True
    if isinstance(c1, Plain) and isinstance(c2, Template):
        return True  # OVL-27
    if isinstance(c1, Template) and isinstance(c2, Template):
        a = at_least_as_specialized(c1, c2, env)
        bb = at_least_as_specialized(c2, c1, env)
        return a and not bb  # OVL-28
    return False
