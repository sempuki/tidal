#!/usr/bin/env python3
"""I6: dependent names and the checks before instantiation, from the rules alone.

Built in the blind phase from templates/dependent-names.md (DEP-P1 to DEP-P6,
DEP-1 to DEP-39), templates/instantiation.md (INST-P1, INST-P2, INST-1 to
INST-13, INST-30, INST-31), the instantiation paragraphs of DED-29, and the
linked rules NAM-27, NAM-37, TARG-2, TARG-23, SA-5, IF-13 and CNS-13, all read
through filter.py.

The input is a small tree. Types, expressions and a template context are
dataclasses below. The main entry points are:

  dependent_type(t, ctx)             DEP-1
  type_dependent(e, ctx)             DEP-2, DEP-3
  value_dependent(e, ctx)            DEP-4
  is_current_instantiation(t, ctx)   DEP-15
  lookup_unqualified(name, ctx)      DEP-11, DEP-12, DEP-14, DEP-36
  binding(e, ctx)                    DEP-5 to DEP-10, DEP-13, DEP-16 to DEP-18, DEP-39
  keywords(q, ctx, position)         DEP-16, DEP-19 to DEP-27, DEP-34
  at_instantiation(q, ctx, member)   DEP-19, DEP-22, DEP-26 (what the second phase checks)
  resolve_call(call, ctx, prog, args) DEP-8 to DEP-10, NAM-27, INST-31: which function a
                                     dependent or independent call finds
  second_lookup(...)                 DEP-13, DEP-35
  pre_checks(ctx)                    DEP-28 to DEP-33, DEP-37, SA-5, IF-13, CNS-13
  analyze(ctx)                       one row per name and expression in ctx.body
  instantiates(entity, use)          INST-1, INST-2, INST-7 to INST-10, INST-3
  made_with_class(member)            INST-5, INST-P2
  error_effect(place)                INST-6, DED-29
  local_class_parts()                INST-12
  friend_redefinition(...)           INST-13

A check before instantiation has one of three strengths:
  "must"  the implementation rejects the template even with no specialization,
  "may"   a no-diagnostic rule: the implementation can reject, need not,
  "none"  nothing is checked before a specialization is made.
"""
from __future__ import annotations

from dataclasses import dataclass, field
from typing import Optional, Tuple, Any

# ======================================================================== types


@dataclass(frozen=True)
class Fund:
    name: str  # int, long, short, char, bool, double, float, void, nullptr_t


@dataclass(frozen=True)
class TParam:
    name: str
    kind: str = "type"          # type | const | template
    ctype: Any = None           # the type of a constant parameter; Placeholder for auto
    pack: bool = False


@dataclass(frozen=True)
class Placeholder:
    name: str = "auto"


@dataclass(frozen=True)
class ClassType:
    """A class that is not a template specialization."""
    name: str
    ns: str = ""
    bases: Tuple[Any, ...] = ()
    outer: Optional[str] = None


@dataclass(frozen=True)
class TemplateRef:
    name: str
    ns: str = ""


@dataclass(frozen=True)
class Spec:
    """A template-id: template<args>. template is a TemplateRef or a TParam."""
    template: Any
    args: Tuple[Any, ...]


@dataclass(frozen=True)
class Ptr:
    to: Any


@dataclass(frozen=True)
class Ref:
    to: Any
    rvalue: bool = False


@dataclass(frozen=True)
class Arr:
    of: Any
    bound: Any = None     # an expression or None


@dataclass(frozen=True)
class Func:
    ret: Any
    params: Tuple[Any, ...] = ()
    pack: bool = False    # a function parameter pack
    noexcept: Any = None  # an expression or None


@dataclass(frozen=True)
class MemPtr:
    cls: Any
    to: Any


@dataclass(frozen=True)
class CV:
    of: Any
    quals: frozenset = frozenset({"const"})


@dataclass(frozen=True)
class Decltype:
    expr: Any


@dataclass(frozen=True)
class Qual:
    """A qualified name qualifier::name, perhaps with typename, template, args."""
    qualifier: Any
    name: str
    kw_typename: bool = False
    kw_template: bool = False
    targs: Optional[Tuple[Any, ...]] = None


@dataclass(frozen=True)
class Injected:
    """The injected class name of the class whose body we are in."""
    nested: Optional[str] = None


@dataclass(frozen=True)
class Nested:
    """A nested class of the current instantiation, by name."""
    name: str
    qualifier: Any = None   # None, or the type it is written after


@dataclass(frozen=True)
class AliasName:
    """A member alias of the context class, used by name (`using self = T`)."""
    name: str


@dataclass(frozen=True)
class AliasSpec:
    """An alias template specialization: alias<args>."""
    name: str
    args: Tuple[Any, ...]


# ================================================================== expressions


@dataclass(frozen=True)
class Lit:
    type: Any = Fund("int")
    value: Any = 0


@dataclass(frozen=True)
class Id:
    name: str
    targs: Optional[Tuple[Any, ...]] = None


@dataclass(frozen=True)
class This:
    pass


@dataclass(frozen=True)
class QualId:
    qualifier: Any
    name: str
    kw_template: bool = False
    targs: Optional[Tuple[Any, ...]] = None


@dataclass(frozen=True)
class Member:
    obj: Any
    name: str
    arrow: bool = False
    kw_template: bool = False
    targs: Optional[Tuple[Any, ...]] = None


@dataclass(frozen=True)
class Call:
    fn: Any
    args: Tuple[Any, ...] = ()


@dataclass(frozen=True)
class Op:
    op: str
    operands: Tuple[Any, ...]


@dataclass(frozen=True)
class Sizeof:
    of: Any            # a type or an expression


@dataclass(frozen=True)
class Alignof:
    of: Any


@dataclass(frozen=True)
class SizeofPack:
    pack: str


@dataclass(frozen=True)
class NoexceptOp:
    expr: Any


@dataclass(frozen=True)
class Cast:
    type: Any
    expr: Any = None
    kind: str = "static"     # static | functional | c | dynamic | const | reinterpret


@dataclass(frozen=True)
class New:
    type: Any
    args: Tuple[Any, ...] = ()


@dataclass(frozen=True)
class Delete:
    expr: Any


@dataclass(frozen=True)
class Throw:
    expr: Any = None


@dataclass(frozen=True)
class Fold:
    op: str
    expr: Any
    init: Any = None


@dataclass(frozen=True)
class Braced:
    elems: Tuple[Any, ...]


@dataclass(frozen=True)
class Expand:
    """A pack expansion `e...` as a call argument."""
    expr: Any


TYPE_NODES = (Fund, TParam, Placeholder, ClassType, Spec, Ptr, Ref, Arr, Func, MemPtr,
              CV, Decltype, Qual, Injected, Nested, AliasName, AliasSpec)
EXPR_NODES = (Lit, Id, This, QualId, Member, Call, Op, Sizeof, Alignof, SizeofPack,
              NoexceptOp, Cast, New, Delete, Throw, Fold, Braced, Expand)


def is_type(x):
    return isinstance(x, TYPE_NODES)


# ================================================================ declarations


@dataclass
class Var:
    """A variable, parameter or structured binding in the template."""
    type: Any
    const: bool = False
    constexpr: bool = False
    init: Any = None
    binding_of: Any = None      # a structured binding: the type of the hidden variable


@dataclass
class MemberDecl:
    """A member that the class declares itself (or an independent base declares)."""
    kind: str                   # value | function | type | alias | template | class | enum
    type: Any = None
    deduced_return: bool = False
    static_const_of: Optional[str] = None   # `static const int M = N;`


@dataclass
class FuncDecl:
    """A namespace-scope function in the program, with its position."""
    name: str
    params: Tuple[Any, ...]
    ns: str = ""
    pos: int = 0
    defaults: int = 0           # the number of trailing parameters with a default
    ret: Any = Fund("int")
    template: bool = False      # a function template that takes any argument
    tag: str = ""               # a label for the caller


@dataclass
class AliasTemplate:
    params: Tuple[str, ...]
    target: Any


@dataclass
class Ctx:
    """A template definition. kind: function | class | member (a member of a class template)."""
    kind: str = "function"
    name: str = ""
    ns: str = ""
    params: Tuple[TParam, ...] = ()
    partial_args: Optional[Tuple[Any, ...]] = None    # a partial specialization's own list
    bases: Tuple[Any, ...] = ()
    members: dict = field(default_factory=dict)       # name -> MemberDecl
    base_members: dict = field(default_factory=dict)  # independent base -> {name: MemberDecl}
    aliases: dict = field(default_factory=dict)       # member alias name -> type
    alias_templates: dict = field(default_factory=dict)
    nested: Optional[str] = None                      # we are inside this nested class
    locals: dict = field(default_factory=dict)        # name -> Var
    globals: dict = field(default_factory=dict)       # name -> kind, declared before the definition
    body: list = field(default_factory=list)          # constructs for analyze() and pre_checks()
    pos: int = 0                                      # the position of the definition
    in_class_template: bool = True

    def param(self, name):
        for p in self.params:
            if p.name == name:
                return p
        return None


# ============================================================ DEP-15: current instantiation


def _resolve_alias(t, ctx):
    if isinstance(t, AliasName) and t.name in ctx.aliases:
        return _resolve_alias(ctx.aliases[t.name], ctx)
    if isinstance(t, AliasSpec) and t.name in ctx.alias_templates:
        at = ctx.alias_templates[t.name]
        return _resolve_alias(substitute(at.target, dict(zip(at.params, t.args))), ctx)
    return t


def _same_arg(a, b, ctx):
    a = _resolve_alias(a, ctx)
    b = _resolve_alias(b, ctx)
    if isinstance(a, Id) and isinstance(b, TParam):
        # a static constant member whose value is a parameter does not count (DEP-15)
        return a.name == b.name and ctx.param(a.name) is not None
    if isinstance(b, Id) and isinstance(a, TParam):
        return _same_arg(b, a, ctx)
    return a == b


def is_current_instantiation(t, ctx):
    """DEP-15: does t name the current instantiation (or, with ctx.nested, the nested class)?"""
    if not ctx.in_class_template or ctx.kind == "function":
        return False
    t = _resolve_alias(t, ctx)
    if isinstance(t, Injected):
        return True
    if isinstance(t, Nested):
        return ctx.nested == t.name and (t.qualifier is None or is_current_instantiation(t.qualifier, _outer(ctx)))
    if isinstance(t, Qual) and ctx.nested and t.name == ctx.nested:
        return is_current_instantiation(t.qualifier, _outer(ctx))
    if isinstance(t, Spec) and isinstance(t.template, TemplateRef) and t.template.name == ctx.name:
        if ctx.nested:
            return False
        own = ctx.partial_args if ctx.partial_args is not None else tuple(ctx.params)
        if len(own) != len(t.args):
            return False
        return all(_same_arg(a, o, ctx) for a, o in zip(t.args, own))
    return False


def _outer(ctx):
    o = Ctx(**{k: getattr(ctx, k) for k in ctx.__dataclass_fields__})
    o.nested = None
    return o


def names_enclosing_current(t, ctx):
    return is_current_instantiation(t, ctx) or (ctx.nested is not None and is_current_instantiation(t, _outer(ctx)))


def has_dependent_base(ctx):
    return any(dependent_type(b, ctx) and not names_enclosing_current(b, ctx) for b in ctx.bases)


# ================================================================== DEP-1: dependent types


def dependent_type(t, ctx):
    t0 = t
    if isinstance(t, AliasName):
        if t.name in ctx.aliases:
            # a member alias of the current instantiation: dependent only if its target is
            return dependent_type(ctx.aliases[t.name], ctx)
        return False
    if isinstance(t, AliasSpec):
        return dependent_type(_resolve_alias(t, ctx), ctx) if t.name in ctx.alias_templates else \
            any(_dep_arg(a, ctx) for a in t.args)
    if isinstance(t, TParam):
        return True
    if isinstance(t, (Fund, ClassType, Placeholder)):
        return False
    if isinstance(t, (Ptr, Ref, CV)):
        return dependent_type(t.to if not isinstance(t, CV) else t.of, ctx)
    if isinstance(t, Arr):
        return dependent_type(t.of, ctx) or (t.bound is not None and value_dependent(t.bound, ctx))
    if isinstance(t, Func):
        return (t.pack or dependent_type(t.ret, ctx) or any(dependent_type(p, ctx) for p in t.params)
                or (t.noexcept is not None and value_dependent(t.noexcept, ctx)))
    if isinstance(t, MemPtr):
        return dependent_type(t.cls, ctx) or dependent_type(t.to, ctx)
    if isinstance(t, Injected) or isinstance(t, Nested):
        return ctx.in_class_template and ctx.kind != "function"
    if isinstance(t, Spec):
        if is_current_instantiation(t, ctx):
            return True
        if isinstance(t.template, TParam):
            return True
        return any(_dep_arg(a, ctx) for a in t.args)
    if isinstance(t, Decltype):
        return type_dependent(t.expr, ctx)
    if isinstance(t, Qual):
        q = t.qualifier
        if names_enclosing_current(q, ctx):
            m = ctx.members.get(t.name) or _base_member(t.name, ctx)
            if m is not None:
                if m.kind in ("class", "enum"):
                    return True      # a nested class or enumeration of the current instantiation
                if m.kind == "alias":
                    return dependent_type(m.type, ctx)
                return False
            return has_dependent_base(ctx)   # DEP-18: waits for the instantiation
        return dependent_type(q, ctx)
    raise TypeError(t0)


def _dep_arg(a, ctx):
    if isinstance(a, Expand):
        return True
    if is_type(a):
        return dependent_type(a, ctx)
    return value_dependent(a, ctx)


def _base_member(name, ctx):
    for b, mems in ctx.base_members.items():
        if name in mems:
            return mems[name]
    return None


# ============================================================ DEP-2, DEP-3: type-dependent


def type_dependent(e, ctx):
    if isinstance(e, Lit):
        return False
    if isinstance(e, (Sizeof, Alignof, SizeofPack, NoexceptOp, Delete, Throw)):
        return False                                       # DEP-3
    if isinstance(e, (Cast, New)):
        return dependent_type(e.type, ctx)                 # DEP-2 and DEP-3
    if isinstance(e, This):
        return ctx.in_class_template and ctx.kind != "function"
    if isinstance(e, Fold):
        return True
    if isinstance(e, Expand):
        return type_dependent(e.expr, ctx)
    if isinstance(e, Braced):
        return any(type_dependent(x, ctx) for x in e.elems)
    if isinstance(e, Id):
        if e.targs and any(_dep_arg(a, ctx) for a in e.targs):
            return True
        p = ctx.param(e.name)
        if p is not None:
            return p.kind == "const" and (isinstance(p.ctype, Placeholder) or
                                          (p.ctype is not None and dependent_type(p.ctype, ctx)))
        v = ctx.locals.get(e.name)
        if v is not None:
            if v.binding_of is not None:
                return dependent_type(v.binding_of, ctx)
            if isinstance(v.type, Placeholder):
                return v.init is not None and type_dependent(v.init, ctx)
            return dependent_type(v.type, ctx)
        m = ctx.members.get(e.name) or _base_member(e.name, ctx)
        if m is not None:
            return _member_type_dependent(m, ctx)
        return False
    if isinstance(e, QualId):
        if e.targs and any(_dep_arg(a, ctx) for a in e.targs):
            return True
        q = e.qualifier
        if names_enclosing_current(q, ctx):
            m = ctx.members.get(e.name) or _base_member(e.name, ctx)
            if m is None:
                return has_dependent_base(ctx)
            return _member_type_dependent(m, ctx)
        return dependent_type(q, ctx)                      # an overload set or a value through T::
    if isinstance(e, Member):
        if e.targs and any(_dep_arg(a, ctx) for a in e.targs):
            return True
        if _object_is_current(e, ctx):
            m = ctx.members.get(e.name) or _base_member(e.name, ctx)
            if m is None:
                return has_dependent_base(ctx)
            return _member_type_dependent(m, ctx)
        return type_dependent(e.obj, ctx)
    if isinstance(e, Call):
        # an unqualified call with a type-dependent argument finds its function at the
        # instantiation (DEP-8), so the type of the call is not known (DEP-2, last bullet)
        return type_dependent(e.fn, ctx) or any(type_dependent(a, ctx) for a in e.args)
    if isinstance(e, Op):
        return any(type_dependent(x, ctx) for x in e.operands)
    raise TypeError(e)


def _member_type_dependent(m, ctx):
    if m.kind == "function":
        return m.deduced_return or (m.type is not None and dependent_type(m.type, ctx))
    if m.type is None:
        return False
    return dependent_type(m.type, ctx)


def _object_is_current(e, ctx):
    o = e.obj
    if isinstance(o, This):
        return ctx.in_class_template and ctx.kind != "function"
    if isinstance(o, Op) and o.op == "*" and len(o.operands) == 1 and isinstance(o.operands[0], This):
        return True
    if isinstance(o, Id) and o.name in ctx.locals:
        t = ctx.locals[o.name].type
        while isinstance(t, (Ref, Ptr, CV)):
            t = t.to if not isinstance(t, CV) else t.of
        return is_current_instantiation(t, ctx)
    return False


# ================================================================== DEP-4: value-dependent


def value_dependent(e, ctx):
    if is_type(e):
        return False
    if isinstance(e, Lit):
        return False
    if isinstance(e, (SizeofPack, Fold)):
        return True
    if isinstance(e, (Sizeof, Alignof)):
        x = e.of
        return dependent_type(x, ctx) if is_type(x) else type_dependent(x, ctx)
    if isinstance(e, NoexceptOp):
        return type_dependent(e.expr, ctx) or _contains_dependent_call(e.expr, ctx)
    if type_dependent(e, ctx):
        return True
    if isinstance(e, Id):
        p = ctx.param(e.name)
        if p is not None:
            return p.kind == "const"
        v = ctx.locals.get(e.name)
        if v is not None:
            return (v.const or v.constexpr) and v.init is not None and value_dependent(v.init, ctx)
        m = ctx.members.get(e.name)
        if m is not None and m.static_const_of:
            return ctx.param(m.static_const_of) is not None
        return False
    if isinstance(e, Cast):
        return e.expr is not None and value_dependent(e.expr, ctx)
    if isinstance(e, Call):
        fn_vd = False if isinstance(e.fn, Id) else value_dependent(e.fn, ctx)
        return fn_vd or any(value_dependent(a, ctx) for a in e.args)
    if isinstance(e, Op):
        return any(value_dependent(x, ctx) for x in e.operands)
    if isinstance(e, Braced):
        return any(value_dependent(x, ctx) for x in e.elems)
    if isinstance(e, (QualId, Member)):
        if isinstance(e, QualId) and names_enclosing_current(e.qualifier, ctx):
            m = ctx.members.get(e.name)
            return bool(m and m.static_const_of and ctx.param(m.static_const_of))
        return False
    if isinstance(e, (New, Delete, Throw, This, Expand)):
        return False
    return False


def _contains_dependent_call(e, ctx):
    return False


# ======================================================= DEP-11, DEP-12, DEP-14, DEP-36: lookup


@dataclass
class Found:
    where: str          # local | member | base-member | param | global | none
    rule: str
    what: Any = None


def lookup_unqualified(name, ctx, in_dependent_base=False):
    """Ordinary unqualified lookup at the definition. in_dependent_base says whether a
    dependent base would declare the name (the caller knows the arguments)."""
    if name in ctx.locals:
        return Found("local", "DEP-5", ctx.locals[name])
    if ctx.in_class_template and ctx.kind != "function":
        if name in ctx.members:
            return Found("member", "DEP-14" if ctx.param(name) is None else "DEP-36", ctx.members[name])
        m = _base_member(name, ctx)
        if m is not None:
            return Found("base-member", "DEP-11" if ctx.param(name) is None else "DEP-36", m)
    if ctx.param(name) is not None:
        return Found("param", "DEP-5", ctx.param(name))
    if name in ctx.globals:
        return Found("global", "DEP-5", ctx.globals[name])
    if in_dependent_base and has_dependent_base(ctx):
        return Found("none", "DEP-12")
    return Found("none", "DEP-6")


# ================================================================ binding of names


@dataclass
class Binding:
    when: str            # definition | instantiation | instantiation+adl | not-found
    rule: str
    note: str = ""


def binding(e, ctx, in_dependent_base=False):
    """Where the name in e binds."""
    if isinstance(e, Call):
        fn = e.fn
        if isinstance(fn, Id):
            dep = any(type_dependent(a, ctx) or isinstance(a, Expand) or
                      (isinstance(a, Braced) and any(type_dependent(x, ctx) for x in a.elems))
                      for a in e.args)
            dep = dep or bool(fn.targs and any(_dep_arg(a, ctx) for a in fn.targs))
            f = lookup_unqualified(fn.name, ctx, in_dependent_base)
            if f.where in ("member", "base-member", "local"):
                # a member found at the definition keeps the member (DEP-14); member
                # calls take no argument-dependent lookup
                return Binding("definition" if not dep else "instantiation", "DEP-14")
            if dep:
                return Binding("instantiation+adl", "DEP-8", "ordinary lookup at the definition, "
                               "argument-dependent lookup also at the point of instantiation (DEP-9)")
            if f.where == "none":
                return Binding("not-found", f.rule)
            return Binding("definition", "DEP-5")
        if isinstance(fn, (QualId, Member)):
            return binding(fn, ctx, in_dependent_base)
        return Binding("instantiation" if type_dependent(fn, ctx) else "definition", "DEP-7")
    if isinstance(e, Op):
        if any(type_dependent(x, ctx) for x in e.operands):
            return Binding("instantiation+adl", "DEP-9")
        return Binding("definition", "DEP-5")
    if isinstance(e, Id):
        if e.name.startswith("operator ") :
            t = e.targs[0] if e.targs else None
            if t is not None and dependent_type(t, ctx):
                return Binding("instantiation", "DEP-11", "also searches the dependent bases")
            f = lookup_unqualified(e.name, ctx, in_dependent_base)
            if f.where == "none":
                return Binding("not-found", "DEP-12")
            return Binding("definition", "DEP-14")
        f = lookup_unqualified(e.name, ctx, in_dependent_base)
        if f.where == "none":
            return Binding("not-found", f.rule)
        if f.where in ("member", "base-member"):
            return Binding("definition", f.rule)
        return Binding("definition", "DEP-5")
    if isinstance(e, QualId):
        q = e.qualifier
        if names_enclosing_current(q, ctx):
            m = ctx.members.get(e.name) or _base_member(e.name, ctx)
            if m is not None:
                return Binding("definition", "DEP-16",
                               "looked up again at the instantiation (DEP-13)")
            if has_dependent_base(ctx):
                return Binding("instantiation", "DEP-18")
            return Binding("not-found", "DEP-17")
        if dependent_type(q, ctx):
            return Binding("instantiation", "DEP-7")
        return Binding("definition", "DEP-5")
    if isinstance(e, Member):
        if _object_is_current(e, ctx):
            m = ctx.members.get(e.name) or _base_member(e.name, ctx)
            if m is not None:
                return Binding("definition", "DEP-13", "looked up again at the instantiation")
            if has_dependent_base(ctx):
                return Binding("instantiation", "DEP-18")
            return Binding("not-found", "DEP-17")
        if type_dependent(e.obj, ctx):
            return Binding("instantiation", "DEP-39")
        return Binding("definition", "DEP-5")
    return Binding("definition" if not type_dependent(e, ctx) else "instantiation", "DEP-5")


# ============================================================ typename and template keywords

# DEP-20: positions where a qualified name with a dependent qualifier is a type
TYPE_ONLY = {
    "base", "nested-name",                 # a base in a base list, a name before ::
    "alias",                               # the type of an alias declaration
    "ns-variable", "ns-return", "trailing-return",
    "member-type", "member-param", "friend-param",
    "qualified-function-param",            # a function declared with a qualified name
    "default-type-arg", "const-param-type",
    "new", "named-cast",
    "lambda-param",
}
# positions that are not type-only: a local variable, a parameter of a function declared
# with an unqualified name at namespace scope, sizeof, a template argument, a C-style or
# functional cast, a using-declaration, an expression


@dataclass
class KeywordResult:
    dependent: bool
    reads_as: str           # type | value | template-id | less-than | error
    typename: str           # needed | optional | not-allowed | not-needed
    template: str           # needed | optional | not-allowed | not-needed
    errors: tuple = ()      # (rule, strength) pairs, strength must | may
    binds: str = "definition"


def keywords(q, ctx, position="other", intended="type", declares=False, destructor=False,
             declarator_name=None):
    """q is a Qual (a type use) or a QualId/Member (a value or template use).
    intended: what the program means the name to be: type | value | template | template-arg.
    declares: the use is the name that a declaration declares (DEP-27).
    declarator_name: for `T::x * p` or `T::x p` written in a statement, the declared name."""
    errors = []
    if isinstance(q, Member):
        obj_dep = type_dependent(q.obj, ctx)
        cur = _object_is_current(q, ctx)
        if q.targs is not None or intended == "template":
            if q.kw_template:
                if destructor:
                    errors.append(("DEP-34", "must"))
                return KeywordResult(obj_dep, "template-id", "not-allowed", "optional" if not obj_dep or cur else "needed",
                                     tuple(errors), "instantiation" if obj_dep and not cur else "definition")
            if obj_dep and not (cur and (ctx.members.get(q.name) or MemberDecl("none")).kind == "template"):
                return KeywordResult(True, "less-than", "not-allowed", "needed", (("DEP-24", "at-instantiation"),),
                                     "instantiation")
            return KeywordResult(obj_dep, "template-id", "not-allowed", "not-needed", (), "definition")
        return KeywordResult(obj_dep, "value", "not-allowed", "not-allowed", (), binding(q, ctx).when)

    qualifier = q.qualifier
    cur = names_enclosing_current(qualifier, ctx)
    m = (ctx.members.get(q.name) or _base_member(q.name, ctx)) if cur else None
    if cur and m is None and has_dependent_base(ctx):
        dep = True
        cur = False                       # DEP-18: the name is dependent; treat as unknown
    elif cur:
        dep = False
    else:
        dep = dependent_type(qualifier, ctx)

    if declares:
        if q.kw_template:
            errors.append(("DEP-27", "must"))
        return KeywordResult(dep, "declarator", "not-allowed", "not-allowed", tuple(errors))
    if destructor and q.kw_template:
        errors.append(("DEP-34", "must"))

    has_args = q.targs is not None
    # template keyword
    if has_args or intended in ("template", "template-arg"):
        if q.kw_template:
            tkw = "needed" if dep else "optional"
        elif dep:
            tkw = "needed"
            if intended in ("type", "template", "template-arg") or position in ("base", "alias") or q_has_typename(q):
                errors.append(("DEP-24", "must" if has_args else "at-instantiation"))
            else:
                errors.append(("DEP-24", "at-instantiation"))
        else:
            tkw = "not-needed"
    else:
        tkw = "not-allowed" if q_kw_template(q) else "not-needed"
        if q_kw_template(q):
            errors.append(("DEP-26", "must" if not dep else "at-instantiation"))

    # typename keyword
    if not dep:
        tn = "optional"                   # DEP-16, DEP-21
        reads = "type" if (q.kw_typename or (m is not None and m.kind in ("type", "alias", "class", "enum"))
                           or intended == "type") else "value"
        if has_args and (q.kw_template or not dep):
            reads = "template-id" if intended != "type" else "type"
    else:
        if q.kw_typename:
            tn = "needed" if position not in TYPE_ONLY else "optional"
            reads = "type"
        elif position in TYPE_ONLY:
            tn = "optional"
            reads = "type"
        else:
            tn = "needed" if intended == "type" else "not-needed"
            reads = "value"
            if intended == "type":
                # DEP-19: read as a value. `T::x * p;` is a product with an undeclared p;
                # `T::x p;` and `int f(T::x) {..}` break the grammar.
                if declarator_name is not None and declarator_name[0] == "*":
                    errors.append(("DEP-19", "at-instantiation"))
                    errors.append(("DEP-6", "must"))
                elif declarator_name is not None:
                    errors.append(("DEP-29", "must"))
                elif position == "ns-function-param-with-body":
                    errors.append(("DEP-29", "must"))
                else:
                    errors.append(("DEP-19", "at-instantiation"))
        if has_args and q.kw_template:
            reads = "type" if (q.kw_typename or position in TYPE_ONLY) else "template-id"
        elif has_args:
            reads = "less-than"
    if q.kw_typename and not isinstance(q, Qual):
        errors.append(("DEP-21", "must"))
    return KeywordResult(dep, reads, tn, tkw, tuple(errors), "instantiation" if dep else "definition")


def q_has_typename(q):
    return getattr(q, "kw_typename", False)


def q_kw_template(q):
    return getattr(q, "kw_template", False)


def typename_unqualified():
    """DEP-21: the name after typename is always a qualified name."""
    return ("DEP-21", "must")


def using_declaration(with_typename, used_as):
    """DEP-23: a using-declaration of a member of a dependent base."""
    if with_typename:
        return "type"
    return "value" if used_as == "value" else ("DEP-23", "reject at the use")


def at_instantiation(q, member_kind, read_as=None):
    """The second phase for a qualified name with a dependent qualifier.
    member_kind is what the name finds for the arguments: type | value | template |
    class-template | alias-template | function-template | none.
    read_as is what the first phase read (from keywords().reads_as)."""
    if read_as is None:
        read_as = "type" if getattr(q, "kw_typename", False) else "value"
    errs = []
    if getattr(q, "kw_template", False) and member_kind not in ("template", "class-template",
                                                                 "alias-template", "function-template"):
        errs.append("DEP-26")
    if read_as == "type":
        if member_kind not in ("type", "class-template", "alias-template") or (
                member_kind in ("class-template", "alias-template") and getattr(q, "targs", None) is None
                and not getattr(q, "kw_template", False)):
            if member_kind not in ("type",):
                errs.append("DEP-22")
    elif read_as == "value":
        if member_kind == "type":
            errs.append("DEP-19")
        elif member_kind == "none":
            errs.append("NAM")
    elif read_as == "less-than":
        if member_kind in ("template", "class-template", "alias-template", "function-template"):
            errs.append("DEP-24")
    return tuple(errs)


# =============================================================== DEP-8 to DEP-10: call lookup


def substitute(t, env):
    """Put arguments in place of parameters (names -> types or values)."""
    if isinstance(t, TParam):
        return env.get(t.name, t)
    if isinstance(t, Id) and t.name in env:
        return env[t.name]
    if isinstance(t, tuple):
        return tuple(substitute(x, env) for x in t)
    if hasattr(t, "__dataclass_fields__") and getattr(t, "__dataclass_params__").frozen:
        kw = {k: substitute(getattr(t, k), env) for k in t.__dataclass_fields__}
        return type(t)(**kw)
    return t


def associated_namespaces(t, classes=None):
    """NAM-27, for the type of an argument at the instantiation."""
    out = set()
    if isinstance(t, (Fund,)) or t is None:
        return out
    if isinstance(t, CV):
        return associated_namespaces(t.of, classes)
    if isinstance(t, (Ptr, Ref)):
        return associated_namespaces(t.to, classes)
    if isinstance(t, Arr):
        return associated_namespaces(t.of, classes)
    if isinstance(t, Func):
        for p in t.params + (t.ret,):
            out |= associated_namespaces(p, classes)
        return out
    if isinstance(t, MemPtr):
        return associated_namespaces(t.cls, classes) | associated_namespaces(t.to, classes)
    if isinstance(t, ClassType):
        out.add(t.ns)
        for b in t.bases:
            out |= associated_namespaces(b, classes)
        return out
    if isinstance(t, Spec):
        out.add(t.template.ns if isinstance(t.template, TemplateRef) else "")
        for a in t.args:
            if is_type(a):
                out |= associated_namespaces(a, classes)
        return out
    return out


def _decay(t):
    if isinstance(t, Ref):
        t = t.to
    if isinstance(t, CV):
        t = t.of
    return t


INT_RANK = {"bool": 0, "char": 1, "short": 2, "int": 3, "long": 4, "long long": 5}
FLOATS = {"float", "double", "long double"}


def conversion_rank(arg, param):
    """A small model of implicit conversion sequences. 3 exact, 2 promotion,
    1 conversion, 0 not viable. Outside knowledge (notes 9)."""
    a, p = _decay(arg), _decay(param)
    if p == "any":
        return 3
    if a == p:
        return 3
    if isinstance(a, Fund) and isinstance(p, Fund):
        if a.name == "void" or p.name == "void":
            return 0
        if p.name == "int" and a.name in ("bool", "char", "short"):
            return 2
        if p.name == "double" and a.name == "float":
            return 2
        if a.name in INT_RANK or a.name in FLOATS:
            if p.name in INT_RANK or p.name in FLOATS:
                return 1
        return 0
    if isinstance(a, ClassType) and isinstance(p, ClassType):
        if p in _all_bases(a):
            return 1
        return 0
    if isinstance(a, Ptr) and isinstance(p, Ptr):
        if _decay(p.to) == Fund("void"):
            return 1
        if isinstance(_decay(a.to), ClassType) and _decay(p.to) in _all_bases(_decay(a.to)):
            return 1
        if isinstance(p.to, CV) and p.to.of == a.to:
            return 3
        return 0
    return 0


def _all_bases(c):
    out = []
    for b in getattr(c, "bases", ()):
        out.append(b)
        out += _all_bases(b)
    return out


@dataclass
class CallResult:
    chosen: Optional[FuncDecl]
    rule: str
    seen: tuple = ()
    may: tuple = ()     # no-diagnostic rules that may apply


def _ns_visible(decl_ns, from_ns):
    # ordinary lookup from from_ns sees from_ns and its enclosing namespaces
    if decl_ns == "":
        return True
    return from_ns == decl_ns or from_ns.startswith(decl_ns + "::")


def resolve_call(name, arg_types, ctx, prog, dependent, poi, end, first_poi=None, qualified_ns=None):
    """Which function an unqualified (or, with qualified_ns, a qualified) call finds.
    prog: FuncDecl list with positions. ctx.pos is the definition. poi is the point of
    instantiation that counts; end is the end of the translation unit (INST-31).
    arg_types are the types after substitution for a dependent call, or the types at the
    definition for an independent one."""
    if qualified_ns is not None:
        cands = [f for f in prog if f.name == name and f.ns == qualified_ns and
                 f.pos < (ctx.pos if not dependent else max(poi, end))]
        return _choose(cands, arg_types, "DEP-7")
    ordinary = [f for f in prog if f.name == name and f.pos < ctx.pos and _ns_visible(f.ns, ctx.ns)]
    if not dependent:
        return _choose(ordinary, arg_types, "DEP-5")
    nss = set()
    for a in arg_types:
        nss |= associated_namespaces(a)
    last = max(poi, end) if end is not None else poi
    adl = [f for f in prog if f.name == name and f.ns in nss and f.pos < last]
    late = [f for f in adl if f.pos >= (first_poi if first_poi is not None else poi)]
    cands = list({id(f): f for f in ordinary + adl}.values())
    r = _choose(cands, arg_types, "DEP-9")
    if late:
        r.may = r.may + ("INST-32",)
    if r.chosen is None and any(f.name == name and f.pos >= ctx.pos and f not in adl for f in prog):
        r.rule = "DEP-10"
    return r


def _choose(cands, arg_types, rule):
    viable = []
    for f in cands:
        n = len(arg_types)
        if f.template:
            viable.append((f, (3,) * n))
            continue
        if not (len(f.params) - f.defaults <= n <= len(f.params)):
            continue
        ranks = tuple(conversion_rank(a, p) for a, p in zip(arg_types, f.params))
        if all(r > 0 for r in ranks):
            viable.append((f, ranks))
    if not viable:
        return CallResult(None, rule if rule != "DEP-9" else "DEP-10", tuple(cands))
    best = []
    for f, r in viable:
        if all(not _better(r2, r) for f2, r2 in viable if f2 is not f):
            best.append(f)
    # prefer a non-template on a tie
    if len(best) > 1:
        nt = [f for f in best if not f.template]
        if len(nt) == 1:
            best = nt
    if len(best) != 1:
        return CallResult(None, "OVL-ambiguous", tuple(cands))
    return CallResult(best[0], rule, tuple(cands))


def _better(a, b):
    return all(x >= y for x, y in zip(a, b)) and any(x > y for x, y in zip(a, b))


def second_lookup(first, second):
    """DEP-13 and DEP-35: a qualified name into the current instantiation, or a member
    access through this, found `first` at the definition; the lookup at the instantiation
    finds `second` (a list of members). Returns (rule, strength)."""
    if len(second) > 1:
        return ("DEP-13", "must")
    if len(second) == 1 and first is not None and second[0] != first:
        return ("DEP-35", "may")
    return (None, "none")


# ==================================================== DEP-28 to DEP-33, DEP-37: checks before


@dataclass
class Construct:
    """One construct of a template, for pre_checks().
    kind: expr | name | syntax | static-assert | default-arg | override | operator-pack |
          base-pack-twice | constrained-id | discarded | decl
    dependent: None to compute from node, else a bool.
    valid: all | some | none  (which argument lists make it valid; for an independent
           construct, some == all)."""
    kind: str
    node: Any = None
    dependent: Optional[bool] = None
    valid: str = "all"
    used: bool = True                    # a default argument: some call uses it
    later_change: bool = False           # DEP-31: its meaning changes before the instantiation
    op: str = ""                         # operator-pack: the operator
    fixed_params: int = 0
    member: bool = False
    label: str = ""


# arity of operator functions; outside knowledge (notes 19)
UNARY_ONLY = {"!", "~"}
BINARY_ONLY = {"=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=", "==", "!=", "<", ">",
               "<=", ">=", "<=>", "&&", "||", "/", "%", "|", "^", "<<", ">>", ",", "->*", "[]"}
EITHER = {"+", "-", "*", "&"}
INCDEC = {"++", "--"}
ANY_ARITY = {"()"}


def operator_max_params(op, member):
    if op in ANY_ARITY:
        return None
    if op == "[]":
        return None  # a subscript operator can take several parameters in C++23
    if op in UNARY_ONLY:
        n = 1
    elif op in BINARY_ONLY or op in EITHER or op in INCDEC:
        n = 2
    else:
        n = 2
    return n - (1 if member else 0)


def pre_checks(ctx, instantiated=False):
    """For each construct in ctx.body: (label, rule, strength).
    strength must: rejected at the definition; may: a no-diagnostic rule lets the
    implementation reject; none: nothing is checked before a specialization."""
    out = []
    for c in ctx.body:
        out.append((c.label or c.kind,) + _check_one(c, ctx, instantiated))
    return out


def _check_one(c, ctx, instantiated):
    k = c.kind
    if k == "syntax":
        return ("DEP-29", "must")
    if k == "discarded":
        # IF-13: a discarded statement is not instantiated; DEP-32 still applies to it
        if c.valid == "none":
            return ("DEP-32", "may")
        return (None, "none")
    if k == "static-assert":
        return ("SA-5", "none")
    if k == "operator-pack":
        mx = operator_max_params(c.op, c.member)
        if mx is not None and c.fixed_params >= mx:
            return ("DEP-37", "must")
        return (None, "none")
    if k == "base-pack-twice":
        return ("DEP-33", "may")
    if k == "pack-only-empty":
        return ("DEP-33", "may")
    if k == "constrained-id":
        dep = c.dependent if c.dependent is not None else any(_dep_arg(a, ctx) for a in c.node.args)
        if not dep and c.valid != "all":
            return ("CNS-13", "must")
        return (None, "none")
    if k == "override":
        if c.dependent:   # a base depends on the parameters
            return (None, "none")
        return ("DEP-30", "may") if c.valid != "all" else (None, "none")
    if k == "default-arg":
        dep = c.dependent if c.dependent is not None else _node_dependent(c.node, ctx)
        if dep or c.valid == "all":
            return (None, "none")
        return ("DEP-30", "may")
    if k == "name":
        # an unqualified name that the definition does not find
        b = binding(c.node, ctx, in_dependent_base=bool(c.dependent))
        if b.when == "not-found":
            if b.rule == "DEP-17":
                return ("DEP-17", "may")      # DEP-30 applies (DEP-17)
            return (b.rule, "must")
        return (None, "none")
    # expr and decl
    dep = c.dependent if c.dependent is not None else _node_dependent(c.node, ctx)
    if c.later_change and not dep:
        return ("DEP-31", "may")
    if not dep:
        if c.valid == "all":
            return (None, "none")
        return ("DEP-30", "may")
    if c.valid == "none":
        return ("DEP-32", "may")
    return ("DEP-28", "none")


def _node_dependent(n, ctx):
    if n is None:
        return False
    if is_type(n):
        return dependent_type(n, ctx)
    return type_dependent(n, ctx) or value_dependent(n, ctx)


def analyze(ctx):
    """One row per expression or name node in ctx.body that has a node."""
    rows = []
    for c in ctx.body:
        n = c.node
        if n is None:
            continue
        if is_type(n):
            rows.append(dict(label=c.label, dependent_type=dependent_type(n, ctx)))
            continue
        b = binding(n, ctx)
        rows.append(dict(label=c.label, type_dependent=type_dependent(n, ctx),
                         value_dependent=value_dependent(n, ctx), binds=b.when, rule=b.rule))
    return rows


# ====================================================================== instantiation

CLASS_USES_YES = {
    "object", "sizeof", "alignof", "pointer-arithmetic", "member-access", "member-access-pointer",
    "delete", "base-conversion", "function-definition-param", "function-definition-return",
    "call-returning", "deduction-base-search", "qualified-member-name",
    "call-by-value-param",          # INST-P1 table: a call with the class by value
}
CLASS_USES_NO = {
    "pointer-type", "reference-type", "alias", "template-argument", "function-declaration",
    "decltype-call-returning",      # notes 21
}
FUNC_USES_YES = {"call", "address", "explicit-instantiation"}
FUNC_USES_NO = {"decltype-call", "sizeof-call", "noexcept-call", "declaration"}


def instantiates(entity, use, deduced_return=False, constexpr_braced=False, overload_needs=None):
    """Does this use instantiate the entity? Returns True, False or 'unspecified' (INST-3)."""
    if entity == "class":
        if use == "overload-resolution":
            return True if overload_needs else "unspecified"   # INST-3
        if use in CLASS_USES_YES:
            return True
        if use in CLASS_USES_NO:
            return False
        raise ValueError(use)
    if entity == "function":
        if use in FUNC_USES_YES:
            return True
        if use in FUNC_USES_NO:
            return use != "declaration" and (deduced_return or constexpr_braced)
        raise ValueError(use)
    if entity in ("static-member", "variable-template"):
        return use in ("read", "odr-use", "address", "reference-bind")   # INST-8; decltype: no
    if entity == "default-argument":
        return use == "call-omits"                                        # INST-9
    if entity == "default-member-initializer":
        return use in ("constructor-omits", "aggregate-omits")            # INST-10
    if entity == "virtual-function-unused":
        return "unspecified"                                              # INST-11
    if entity == "discarded-statement":
        return False                                                      # IF-13
    if entity == "local-class-parts":
        return use == "function-instantiated"                             # INST-12
    raise ValueError(entity)


MADE_WITH_CLASS = {
    "data-member": "yes",
    "data-member-type": "yes",
    "member-function": "declaration",
    "member-function-body": "no",
    "deleted-function-body": "no",
    "default-argument": "no",
    "exception-specification": "no",
    "default-member-initializer": "no",
    "static-data-member": "declaration",
    "static-inline-initializer": "no",
    "static-constexpr-initializer": "yes",   # INST-33 says it is instantiated with the class
    "nested-class": "declaration",
    "member-template": "declaration",
    "unscoped-enum": "yes",
    "scoped-enum": "declaration",
    "friend-function-definition": "yes",     # INST-13: defined again by each instantiation
    "virtual-function-body": "unspecified",  # INST-11
    "member-requires-clause": "declaration",
}


def made_with_class(member):
    return MADE_WITH_CLASS[member]


def error_effect(place, during_deduction=False):
    """INST-6, INST-P2, DED-29: what an error in an instantiated place does.
    hard: the program is not valid (a hard error, also during deduction);
    use-only: only the use that needs the definition is not valid;
    sfinae: a substitution failure (only in the immediate function type);
    none: no error (the place is not instantiated)."""
    made = MADE_WITH_CLASS.get(place)
    if place in ("function-type", "explicit-template-argument") and during_deduction:
        return "sfinae"
    if place == "losing-candidate-body":
        return "none"
    if place in ("class-body-in-substitution", "body-needed-for-deduced-type", "lambda-in-function-type",
                 "chosen-noexcept"):
        return "hard"
    if made == "yes" or made == "declaration":
        return "hard"
    if made == "no":
        return "use-only"
    if made == "unspecified":
        return "unspecified"
    raise ValueError(place)


def local_class_parts():
    """INST-12: the parts of a local class of a function template made with the function."""
    return {"member-function-body": True, "exception-specification": True,
            "default-member-initializer": True, "default-argument": True,
            "requires-clause": False}


def friend_redefinition(friend_depends_on_params, specializations):
    """INST-13: a friend function defined in a class template. Returns the rule if two
    instantiated specializations define the same function."""
    if friend_depends_on_params:
        return None
    return "INST-13" if specializations >= 2 else None


def point_of_instantiation(entity, use_pos, enclosing_point=None, end=None):
    """INST-30 and INST-31. Positions are integers. A class's point is right before
    (use_pos - 0.5) the declaration that needs it, or before the enclosing specialization's
    point. A function's points are right after the declaration, and the end."""
    if entity == "class":
        base = enclosing_point if enclosing_point is not None else use_pos
        return (base - 0.5,)
    pts = [(enclosing_point if enclosing_point is not None else use_pos) + 0.5]
    if end is not None:
        pts.append(end)
    return tuple(pts)
