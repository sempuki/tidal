"""Ranking of implicit conversion sequences and choice of the best viable
function, written from Tidal's pages conversion-sequences.md (ICS) and
overload-resolution.md (OVL), with Evidence segments and :tests: lines removed.

Model
-----
Types are `Ty` values made by the helpers below (fund, cls, enum, ptr, lref,
rref, arr, fnty, mptr, ilist, cq).  Classes and enumerations live in a
`World`.  Arguments are `Arg` values: a type and a value category, or a
braced list of `Arg`s.  Candidates are `Fn` values.

`resolve(world, cands, args, ...)` returns a `Result` whose `status` is
"ok", "ambiguous", or "no viable function", and whose `fn` is the chosen
candidate.  Faults that ICS-8 lists (deleted, access, narrowing, explicit in
copy-list-initialization) do not change the choice; the caller checks them.

Partial ordering (ORD) and constraints are black boxes: `order(f1, f2)`
returns "first", "second", "equal" (each at least as specialized as the
other), or None; `more_constrained(f1, f2)` returns True when f1 is more
constrained than f2 and the two meet the ORD-13 conditions.
"""
from __future__ import annotations

from dataclasses import dataclass, field, replace
from typing import Optional

# ---------------------------------------------------------------- types

INTEGRAL = ['bool', 'char', 'signed char', 'unsigned char', 'wchar_t',
            'char8_t', 'char16_t', 'char32_t', 'short', 'unsigned short',
            'int', 'unsigned', 'long', 'unsigned long', 'long long',
            'unsigned long long']
FLOATING = ['float', 'double', 'long double']
FLOAT_RANK = {'float': 1, 'double': 2, 'long double': 3}
# CONV-1 and the glossary entry "integer promotion", matrix targets.
PROMOTE = {'bool': 'int', 'char': 'int', 'signed char': 'int',
           'unsigned char': 'int', 'short': 'int', 'unsigned short': 'int',
           'wchar_t': 'int', 'char8_t': 'int', 'char16_t': 'int',
           'char32_t': 'unsigned'}
E = frozenset()


@dataclass(frozen=True)
class Ty:
    kind: str                     # fund enum class ptr mptr ref array func ilist
    name: str = ''                # fund, enum, class name; mptr class
    cv: frozenset = E             # subset of {'const', 'volatile'}
    to: Optional['Ty'] = None     # pointee, referenced, element, ilist element
    bound: Optional[int] = None   # array bound, None for unknown
    rk: str = ''                  # 'l' or 'r' for a reference
    params: tuple = ()
    noexcept: bool = False

    def __repr__(self):
        return show(self)


def show(t):
    if t is None:
        return 'None'
    c = ''.join(q + ' ' for q in sorted(t.cv))
    if t.kind in ('fund', 'enum', 'class'):
        return c + t.name
    if t.kind == 'ptr':
        return f'{show(t.to)}*' + (' ' + ' '.join(sorted(t.cv)) if t.cv else '')
    if t.kind == 'ref':
        return f'{show(t.to)}' + ('&' if t.rk == 'l' else '&&')
    if t.kind == 'array':
        return f'{show(t.to)}[{"" if t.bound is None else t.bound}]'
    if t.kind == 'mptr':
        return f'{show(t.to)} {t.name}::*'
    if t.kind == 'func':
        return f'{show(t.to)}({", ".join(map(show, t.params))})' + (' noexcept' if t.noexcept else '')
    if t.kind == 'ilist':
        return f'std::initializer_list<{show(t.to)}>'
    return t.kind


def fund(n, cv=E):
    return Ty('fund', n, frozenset(cv))


def cls(n, cv=E):
    return Ty('class', n, frozenset(cv))


def enum(n, cv=E):
    return Ty('enum', n, frozenset(cv))


def ptr(t, cv=E):
    return Ty('ptr', cv=frozenset(cv), to=t)


def mptr(c, t, cv=E):
    return Ty('mptr', c, frozenset(cv), to=t)


def lref(t):
    return Ty('ref', to=t, rk='l')


def rref(t):
    return Ty('ref', to=t, rk='r')


def arr(t, n=None):
    return Ty('array', to=t, bound=n)


def fnty(ret, params=(), noexcept=False):
    return Ty('func', to=ret, params=tuple(params), noexcept=noexcept)


def ilist(t):
    return Ty('ilist', to=t)


NULLPTR_T = fund('nullptr_t')
VOID = fund('void')


def cq(t, *quals):
    """Add cv-qualifiers.  The cv-qualifiers of an array are its element's."""
    if t.kind == 'array':
        return replace(t, to=cq(t.to, *quals))
    return replace(t, cv=t.cv | frozenset(quals))


def topcv(t):
    return topcv(t.to) if t.kind == 'array' else t.cv


def strip(t):
    """Drop top-level cv-qualifiers (for an array, those of the element)."""
    if t.kind == 'array':
        return replace(t, to=strip(t.to))
    return replace(t, cv=E)


def is_arith(t):
    return t.kind == 'fund' and (t.name in INTEGRAL or t.name in FLOATING)


def is_integral(t):
    return t.kind == 'fund' and t.name in INTEGRAL


def is_float(t):
    return t.kind == 'fund' and t.name in FLOATING


# ---------------------------------------------------------------- world

@dataclass
class EnumInfo:
    name: str
    underlying: Optional[str] = None   # fixed underlying type
    scoped: bool = False
    promoted: str = 'int'              # CONV-1, for an unfixed enumeration


@dataclass
class ClassInfo:
    name: str
    bases: list = field(default_factory=list)
    ctors: list = field(default_factory=list)     # Fn, kind 'ctor'
    convs: list = field(default_factory=list)     # Fn, kind 'conv'
    aggregate: bool = False
    fields: list = field(default_factory=list)    # (name, Ty)
    implicit_copy_move: bool = True
    default_ctor: Optional[bool] = None           # None: work it out


class World:
    def __init__(self):
        self.classes = {}
        self.enums = {}

    def add_class(self, name, **kw):
        c = ClassInfo(name, **kw)
        self.classes[name] = c
        for f in c.ctors:
            f.kind = 'ctor'
            f.owner = name
        for f in c.convs:
            f.kind = 'conv'
            f.owner = name
        if c.implicit_copy_move and not c.aggregate:
            c.ctors.append(Fn(f'{name}(const {name}&)', [lref(cls(name, {'const'}))],
                              kind='ctor', owner=name, implicit=True))
            c.ctors.append(Fn(f'{name}({name}&&)', [rref(cls(name))],
                              kind='ctor', owner=name, implicit=True))
        return c

    def add_enum(self, name, **kw):
        self.enums[name] = EnumInfo(name, **kw)

    def bases_of(self, n):
        c = self.classes.get(n)
        return list(c.bases) if c else []

    def is_base(self, b, d):
        """b is a proper base of d (CONV-14).  Ambiguity and access are faults."""
        todo = list(self.bases_of(d))
        while todo:
            x = todo.pop()
            if x == b:
                return True
            todo.extend(self.bases_of(x))
        return False

    def conv_fns(self, n):
        """Conversion functions of class n, inherited ones included."""
        out, seen = [], set()
        order = [n]
        i = 0
        while i < len(order):
            for b in self.bases_of(order[i]):
                if b not in order:
                    order.append(b)
            i += 1
        for c in order:
            for f in self.classes[c].convs if c in self.classes else []:
                key = f.ret
                if key in seen:
                    continue          # a derived declaration hides it
                seen.add(key)
                out.append(f)
        return out


# ---------------------------------------------------------------- functions and arguments

_ids = iter(range(1, 1 << 30))


@dataclass(eq=False)
class Fn:
    name: str
    params: list = field(default_factory=list)
    ellipsis: bool = False
    defaults: int = 0             # trailing parameters with a default argument
    kind: str = 'free'            # free member static explicit builtin ctor conv
    owner: str = ''               # class of a member, constructor, conversion function
    cv: frozenset = E             # cv-qualifiers of a member function
    refq: str = ''                # '', '&', '&&'
    template: bool = False
    constraints_ok: bool = True
    explicit: bool = False
    deleted: bool = False
    ret: Optional[Ty] = None      # result type of a conversion function
    rewritten: bool = False
    swapped: bool = False
    inherited: bool = False       # inherited constructor
    implicit: bool = False
    std_only: tuple = ()          # parameter indices that take only standard sequences
    uid: int = field(default_factory=lambda: next(_ids))

    def __repr__(self):
        return self.name


@dataclass
class Arg:
    type: Optional[Ty] = None
    cat: str = 'prvalue'          # lvalue xvalue prvalue
    null: bool = False            # a null pointer constant (CONV-8)
    braced: Optional[list] = None
    designators: Optional[list] = None
    string: bool = False          # a string literal: lvalue array of const char
    overload_set: Optional[list] = None   # list of (Ty func, template?)

    @property
    def rvalue(self):
        return self.cat != 'lvalue'


def lv(t):
    return Arg(t, 'lvalue')


def xv(t):
    return Arg(t, 'xvalue')


def pr(t):
    return Arg(t, 'prvalue')


def braced(*elems, designators=None):
    return Arg(None, 'prvalue', braced=list(elems), designators=designators)


def strlit(n_chars, ch='char'):
    return Arg(arr(fund(ch, {'const'}), n_chars + 1), 'lvalue', string=True)


# ---------------------------------------------------------------- sequences

EXACT, PROMO, CONV = 3, 2, 1
RANK_NAME = {EXACT: 'exact match', PROMO: 'promotion', CONV: 'conversion'}


@dataclass
class RefInfo:
    rk: str                    # 'l' or 'r'
    referenced: Ty
    to_rvalue: bool            # binds to an rvalue (the argument or a temporary)
    to_function: bool = False  # binds to a function lvalue
    direct: bool = False
    implicit_obj_noref: bool = False   # ICS-40


@dataclass
class Std:
    """A standard conversion sequence (ICS-1, ICS-2)."""
    src: Optional[Ty] = None
    dst: Optional[Ty] = None
    ltr: str = ''              # lvalue transformation
    conv: str = ''             # second step: int_prom float_prom enum_prom_u enum_prom_p
                               # integral float float_integral ptr_null ptr_void ptr_base
                               # mptr_base bool d2b
    conv_dst: Optional[Ty] = None
    qual: bool = False         # third step: qualification adjustment
    rank: int = EXACT
    ref: Optional[RefInfo] = None

    def steps(self):
        s = []
        if self.conv:
            s.append((self.conv, self.conv_dst))
        if self.qual:
            s.append(('qual', self.dst))
        return s


@dataclass
class ICS:
    kind: str                  # standard user ambiguous ellipsis
    std: Optional[Std] = None
    first: Optional[Std] = None
    func: Optional[Fn] = None
    aggregate: str = ''
    third: Optional[Std] = None
    is_list: bool = False
    to_ilist: bool = False                     # ICS-22
    to_array: Optional[tuple] = None           # (element Ty, bound, known) ICS-23, ICS-24
    static_slot: bool = False                  # ICS-14
    faults: list = field(default_factory=list)

    def __repr__(self):
        if self.kind == 'standard':
            s = self.std
            return f'std[{RANK_NAME[s.rank]}{"," + s.conv if s.conv else ""}{",qual" if s.qual else ""}{",ref" + s.ref.rk if s.ref else ""}]'
        if self.kind == 'user':
            return f'user[{self.func or self.aggregate}]'
        return self.kind


def std_ics(s, **kw):
    return ICS('standard', std=s, **kw)


# ---------------------------------------------------------------- qualification adjustment

def qual_adjust(a, b):
    """True if pointer-like type a converts to b by a qualification adjustment
    alone (ICS-P2 group 3, CONV-9, CONV-10, CONV-16), and a != b.  Top-level
    cv-qualifiers are ignored."""
    res = _qual(strip(a), strip(b), 0, True)
    return res == 'changed'


def _qual(a, b, level, const_above):
    changed = False
    if level > 0 and a.kind != 'array':
        if not a.cv <= b.cv:
            return None
        if a.cv != b.cv:
            if not const_above:
                return None
            changed = True
    nconst = const_above and (level == 0 or 'const' in topcv(b))
    if a.kind == 'array' and b.kind == 'array':
        if a.bound != b.bound:
            if b.bound is not None or not const_above:
                return None
            changed = True
        r = _qual(a.to, b.to, level, const_above)
        if r is None:
            return None
        return 'changed' if changed or r == 'changed' else 'same'
    if a.kind != b.kind:
        return None
    if a.kind == 'ptr' or (a.kind == 'mptr' and a.name == b.name):
        r = _qual(a.to, b.to, level + 1, nconst)
    elif a.kind == 'func':
        if a.to != b.to or a.params != b.params:
            return None
        if a.noexcept and not b.noexcept:
            changed = True
        elif a.noexcept != b.noexcept:
            return None
        r = 'same'
    else:
        r = 'same' if strip(a) == strip(b) else None
    if r is None:
        return None
    return 'changed' if changed or r == 'changed' else 'same'


def similar_same(a, b):
    return _qual(strip(a), strip(b), 0, True) == 'same'


# ---------------------------------------------------------------- standard conversions

def std_conv(w, src, cat, dst, null=False):
    """A standard sequence from an expression of type src to type dst (both
    not references), or None."""
    s = Std(src=src, dst=dst)
    cur = src
    if cur.kind == 'array' and dst.kind in ('ptr', 'fund'):
        s.ltr = 'array_to_pointer'
        cur = ptr(cur.to)
    elif cur.kind == 'func' and dst.kind in ('ptr', 'fund'):
        s.ltr = 'function_to_pointer'
        cur = ptr(cur)
    elif cat == 'lvalue' and cur.kind not in ('array', 'func', 'class'):
        s.ltr = 'lvalue_to_rvalue'
    if cur.kind in ('array', 'func') or dst.kind in ('array', 'func', 'ref', 'ilist'):
        return None
    a, b = strip(cur), strip(dst)
    if a == b:
        return s                                    # identity (ICS-1: top-level cv add nothing)
    # classes (ICS-2)
    if a.kind == 'class' or b.kind == 'class':
        if a.kind == 'class' and b.kind == 'class' and w.is_base(b.name, a.name):
            s.conv, s.conv_dst, s.rank = 'd2b', b, CONV
            return s
        return None
    # promotions
    if a.kind == 'fund' and a.name in PROMOTE and b.kind == 'fund' and PROMOTE[a.name] == b.name:
        s.conv, s.conv_dst, s.rank = 'int_prom', b, PROMO
        return s
    if a == fund('float') and b == fund('double'):
        s.conv, s.conv_dst, s.rank = 'float_prom', b, PROMO
        return s
    if a.kind == 'enum':
        info = w.enums.get(a.name, EnumInfo(a.name))
        if info.scoped:
            return None
        if b.kind == 'fund':
            if info.underlying:
                u = info.underlying
                if b.name == u:
                    s.conv, s.conv_dst, s.rank = 'enum_prom_u', b, PROMO
                    return s
                if PROMOTE.get(u) == b.name:
                    s.conv, s.conv_dst, s.rank = 'enum_prom_p', b, PROMO
                    return s
            elif b.name == info.promoted:
                s.conv, s.conv_dst, s.rank = 'int_prom', b, PROMO
                return s
            if b.name == 'bool':
                s.conv, s.conv_dst, s.rank = 'bool', b, CONV
                return s
            if is_arith(b):
                s.conv, s.conv_dst, s.rank = ('integral' if is_integral(b) else 'float_integral'), b, CONV
                return s
        return None
    if b.kind == 'enum':
        return None
    # arithmetic conversions
    if is_arith(a) and is_arith(b):
        if b.name == 'bool':
            kind = 'bool'
        elif is_integral(a) and is_integral(b):
            kind = 'integral'
        elif is_float(a) and is_float(b):
            kind = 'float'
        else:
            kind = 'float_integral'
        s.conv, s.conv_dst, s.rank = kind, b, CONV
        return s
    # null pointer constants (CONV-8)
    if (null or a == NULLPTR_T) and b.kind in ('ptr', 'mptr'):
        s.conv, s.conv_dst, s.rank = 'ptr_null', b, CONV
        return s
    if null and b == NULLPTR_T:
        s.conv, s.conv_dst, s.rank = 'ptr_null', b, CONV
        return s
    # to bool (CONV-3); nullptr only in direct initialization (CONV-12)
    if b == fund('bool') and a.kind in ('ptr', 'mptr'):
        s.conv, s.conv_dst, s.rank = 'bool', b, CONV
        return s
    if a.kind == 'ptr' and b.kind == 'ptr':
        pa, pb = a.to, b.to
        # derived to base pointer (CONV-14)
        if pa.kind == 'class' and pb.kind == 'class' and w.is_base(pb.name, pa.name):
            mid = ptr(cq(pb, *pa.cv))
            s.conv, s.conv_dst, s.rank = 'ptr_base', mid, CONV
            if strip(mid) != b:
                if not qual_adjust(mid, b):
                    return None
                s.qual = True
            return s
        # to void* (CONV-9)
        if strip(pb) == VOID and pa.kind != 'func' and strip(pa) != VOID:
            mid = ptr(cq(VOID, *topcv(pa)))
            s.conv, s.conv_dst, s.rank = 'ptr_void', mid, CONV
            if strip(mid) != b:
                if not qual_adjust(mid, b):
                    return None
                s.qual = True
            return s
    if a.kind == 'mptr' and b.kind == 'mptr' and a.name != b.name:
        if w.is_base(a.name, b.name):
            mid = mptr(b.name, a.to)
            s.conv, s.conv_dst, s.rank = 'mptr_base', mid, CONV
            if strip(mid) != b:
                if not qual_adjust(mid, b):
                    return None
                s.qual = True
            return s
        return None
    if a.kind in ('ptr', 'mptr') and qual_adjust(a, b):
        s.qual = True
        return s
    return None


# ---------------------------------------------------------------- forming a sequence

@dataclass
class Ctx:
    std_only: bool = False         # ICS-10
    direct: bool = False           # direct initialization (ICS-45)
    object_param: bool = False     # implicit object parameter
    noref_object: bool = False     # implicit object parameter without ref-qualifier
    inner_list_class: str = ''     # ICS-10 last bullet
    rref_related: Optional[Ty] = None   # ICS-12: an rvalue reference to this type


def implicit_seq(w, arg, p, ctx=None):
    """The implicit conversion sequence from arg to a parameter of type p."""
    ctx = ctx or Ctx()
    if arg.overload_set is not None:
        return overload_set_seq(w, arg, p, ctx)
    if arg.braced is not None:
        return list_seq(w, arg, p, ctx)
    if p.kind == 'ref':
        return ref_seq(w, arg, p, ctx)
    if p.kind == 'ilist':
        return None
    s = std_conv(w, arg.type, arg.cat, p, arg.null)
    if s is not None:
        return std_ics(s)
    if ctx.std_only:
        return None
    if strip(arg.type).kind == 'class' or strip(p).kind == 'class':
        return user_conv(w, arg, p, ctx)
    return None


def overload_set_seq(w, arg, p, ctx):
    """ICS-47 with OVL-18 to OVL-20, reduced: function types match by
    equality, with noexcept dropped; a plain function beats a template."""
    target = None
    if p.kind == 'ptr' and p.to.kind == 'func':
        target = p.to
    elif p.kind == 'ref' and p.to.kind == 'func':
        target = p.to
    if target is not None:
        m = [(f, t) for (f, t) in arg.overload_set
             if f == target or (f.noexcept and replace(f, noexcept=False) == target)]
    else:
        m = [(f, t) for (f, t) in arg.overload_set if not t]
    if any(not t for _, t in m):
        m = [(f, t) for (f, t) in m if not t]
    if len(m) != 1:
        return None
    a = Arg(m[0][0], 'lvalue')
    if p.kind == 'ref':
        return ref_seq(w, a, p, ctx)
    s = std_conv(w, a.type, 'lvalue', p)
    return std_ics(s) if s else None


def ref_related(w, r, a):
    """a is r, or a class derived from r, ignoring cv."""
    r, a = strip(r), strip(a)
    if r == a:
        return True
    return r.kind == 'class' and a.kind == 'class' and w.is_base(r.name, a.name)


def ref_compatible(w, r, a):
    """An expression of type a can be bound directly by a reference to r
    (REF-1, ICS-46): same type with no fewer cv-qualifiers, a derived class,
    or a qualification adjustment such as int* to const int* const, or an
    array of known bound for one of unknown bound."""
    if ref_related(w, r, a):
        return topcv(a) <= topcv(r)
    if r.kind == 'array' and a.kind == 'array' and r.bound is None and \
            strip(r.to) == strip(a.to) and topcv(a) <= topcv(r):
        return True
    if r.kind in ('ptr', 'mptr', 'array') and topcv(a) <= topcv(r) and qual_adjust(ptr(a), ptr(r)):
        return True
    return False


def ref_seq(w, arg, p, ctx):
    """ICS-11, ICS-12, ICS-40 with REF-1 to REF-3."""
    r = p.to
    rk = p.rk
    a = arg.type
    const_lref = rk == 'l' and 'const' in topcv(r) and 'volatile' not in topcv(r)
    # reference to function (ICS-12, ICS-29)
    if r.kind == 'func':
        if a.kind == 'func' and arg.cat == 'lvalue':
            if strip(a) == strip(r) or (a.noexcept and replace(a, noexcept=False) == r):
                s = Std(src=a, dst=r, ref=RefInfo(rk, r, False, to_function=True, direct=True))
                return std_ics(s)
        if a.kind == 'class' and not ctx.std_only:
            return ref_user(w, arg, p, ctx)
        return None
    rvalue_ok = rk == 'r' or const_lref or ctx.noref_object
    # direct binding to the argument
    if ref_compatible(w, r, a):
        if arg.cat == 'lvalue':
            if rk == 'r':
                return None                         # ICS-12
            ok = True
        else:
            ok = rvalue_ok
        if not ok:
            return None
        s = Std(src=a, dst=r, ref=RefInfo(rk, r, arg.rvalue, direct=True,
                                          implicit_obj_noref=ctx.noref_object))
        if strip(a).kind == 'class' and strip(a) != strip(r) and strip(r).kind == 'class':
            s.conv, s.conv_dst, s.rank = 'd2b', strip(r), CONV
        return std_ics(s)
    if ref_related(w, r, a):
        return None                                 # cv dropped, no temporary
    # direct binding to the result of a conversion function, then a temporary
    if strip(a).kind == 'class' and not ctx.std_only:
        u = ref_user(w, arg, p, ctx)
        if u is not None:
            return u
    if not rvalue_ok or ctx.object_param:
        return None
    # a temporary of the referenced type (ICS-11, third bullet)
    inner = implicit_seq(w, arg, strip(r), replace(ctx, object_param=False, noref_object=False,
                                                  rref_related=(r if rk == 'r' else None)))
    if inner is None:
        return None
    return attach_ref(inner, RefInfo(rk, r, True, direct=False))


def attach_ref(ics, ri):
    ics = replace(ics)
    if ics.kind == 'standard':
        ics.std = replace(ics.std, ref=ri)
    elif ics.kind == 'user' and ics.third is not None:
        ics.third = replace(ics.third, ref=ri)
    return ics


def ref_user(w, arg, p, ctx):
    """Direct binding to the result of a conversion function (ICS-4 third
    bullet, ICS-11 second bullet, OVL-15).  None if no candidate exists."""
    r, rk = p.to, p.rk
    S = strip(arg.type).name
    cands = []
    for f in w.conv_fns(S):
        if f.explicit and not ctx.direct:
            continue
        ret = f.ret
        if ret.kind == 'ref':
            res_cat = 'lvalue' if ret.rk == 'l' else 'xvalue'
            res = ret.to
        else:
            res_cat, res = 'prvalue', ret
        if r.kind == 'func':
            if ret.kind != 'ref' or strip(res) != strip(r):
                continue
        else:
            if rk == 'l' and res_cat != 'lvalue':
                continue
            if rk == 'r' and res_cat == 'lvalue':
                continue
            if not (ref_related(w, r, res) and topcv(res) <= topcv(r)):
                continue
            if rk == 'l' and not ('const' in topcv(r) and 'volatile' not in topcv(r)) and res_cat != 'lvalue':
                continue
        third = Std(src=res, dst=r, ref=RefInfo(rk, r, res_cat != 'lvalue',
                                                to_function=r.kind == 'func', direct=True))
        if strip(res) != strip(r) and strip(r).kind == 'class':
            third.conv, third.conv_dst, third.rank = 'd2b', strip(r), CONV
        if f.template and third.rank != EXACT:
            continue
        cands.append((f, third))
    if not cands:
        return None
    return choose_user(w, arg, cands, target=p)


def object_seq(w, arg, f, S):
    """The sequence of the implied object argument to the implicit object
    parameter of member f of class S (ICS-13, ICS-40)."""
    owner = S if f.kind == 'conv' else (f.owner or S)
    t = cq(cls(owner), *f.cv)
    p = rref(t) if f.refq == '&&' else lref(t)
    return ref_seq(w, arg, p, Ctx(std_only=True, object_param=True,
                                   noref_object=(f.refq == '')))


def user_conv(w, arg, T, ctx):
    """A user-defined sequence to a non-reference type T (ICS-3, ICS-4,
    ICS-5, ICS-7), or None."""
    cands = []
    Tn = strip(T)
    if Tn.kind == 'class' and Tn.name in w.classes:
        for c in w.classes[Tn.name].ctors:
            if c.explicit and not ctx.direct:
                continue
            if not c.params or len(c.params) - 1 > c.defaults:
                if not (c.params == [] and c.ellipsis):
                    continue
            if c.params and c.params[0].kind == 'ilist':
                continue
            first = implicit_seq(w, arg, c.params[0], Ctx(std_only=True)) if c.params else ICS('ellipsis')
            if first is None:
                continue
            third = Std(src=Tn, dst=Tn)
            cands.append((c, third, first))
    if strip(arg.type).kind == 'class':
        S = strip(arg.type).name
        for f in w.conv_fns(S):
            ret = f.ret
            res_cat = 'prvalue'
            res = ret
            if ret.kind == 'ref':
                res, res_cat = ret.to, ('lvalue' if ret.rk == 'l' else 'xvalue')
            if ctx.rref_related is not None and res_cat == 'lvalue' and \
                    ref_related(w, ctx.rref_related, res):
                continue                            # ICS-12, second bullet
            if f.explicit:
                if not ctx.direct:
                    continue
                # ICS-45
                if Tn.kind == 'class':
                    if strip(res) != Tn:
                        continue
                elif not (strip(res) == Tn or qual_adjust(res, Tn)):
                    continue
            if Tn.kind == 'class':
                if strip(res).kind != 'class':
                    continue
                if not (strip(res) == Tn or w.is_base(Tn.name, strip(res).name)):
                    continue
            third = std_conv(w, res, res_cat, Tn)
            if third is None:
                continue
            if f.template and third.rank != EXACT:
                continue                            # ICS-5
            cands.append((f, third, None))
    if not cands:
        return None
    return choose_user(w, arg, cands, target=T)


def choose_user(w, arg, cands, target):
    """ICS-7: choose one constructor or conversion function."""
    fns, extra = [], {}
    for item in cands:
        f, third = item[0], item[1]
        first = item[2] if len(item) > 2 else None
        if first is None:
            first = object_seq(w, arg, f, strip(arg.type).name)
            if first is None:
                continue
        fns.append(f)
        extra[f.uid] = (first, third)
    if not fns:
        return None
    viable = [Viable(f, [extra[f.uid][0]], result=extra[f.uid][1]) for f in fns]
    best = pick_best(w, viable, target=target)
    if best is None:
        return ICS('ambiguous')
    first, third = extra[best.fn.uid]
    return ICS('user', first=first.std if first.kind == 'standard' else None,
               func=best.fn, third=third)


# ---------------------------------------------------------------- braced lists

def worst(seqs):
    """The worst of several element sequences (ICS-16, ICS-17)."""
    out = seqs[0]
    for s in seqs[1:]:
        if compare_ics(s, out) < 0:
            out = s
        elif compare_ics(s, out) == 0 and s.kind == 'standard' and out.kind == 'standard' \
                and s.std.rank < out.std.rank:
            out = s
    return out


def identity():
    return std_ics(Std())


def list_seq(w, arg, p, ctx):
    elems = arg.braced
    desig = arg.designators
    # references (ICS-20, ICS-38, REF-P3)
    if p.kind == 'ref':
        r = p.to
        if len(elems) == 1 and elems[0].braced is None and not desig:
            e = elems[0]
            if e.overload_set is not None and r.kind == 'func':
                s = overload_set_seq(w, e, p, ctx)
                return mark_list(s)
            if e.type is not None and (
                    ref_related(w, r, e.type)
                    or (r.kind == 'array' and r.bound is None and e.type.kind == 'array'
                        and strip(e.type.to) == strip(r.to))
                    or r.kind == 'func'):
                s = ref_seq(w, e, p, ctx)
                return mark_list(s)
        if r.kind == 'func':
            return None
        const_lref = p.rk == 'l' and 'const' in topcv(r) and 'volatile' not in topcv(r)
        if not (p.rk == 'r' or const_lref):
            return None
        inner = list_seq(w, arg, strip(r) if r.kind != 'array' else r, ctx)
        if inner is None:
            return None
        out = attach_ref(inner, RefInfo(p.rk, r, True))
        out.is_list = True
        return out
    # std::initializer_list<E> (ICS-16)
    if p.kind == 'ilist':
        if desig:
            return None
        seqs = []
        for e in elems:
            s = implicit_seq(w, e, p.to)
            if s is None:
                return None
            seqs.append(s)
        out = replace(worst(seqs)) if seqs else identity()
        out.is_list, out.to_ilist = True, True
        return out
    # array (ICS-17, ICS-38)
    if p.kind == 'array':
        if desig:
            return None
        if len(elems) == 1 and elems[0].string and p.to.kind == 'fund':
            e = elems[0]
            n = e.type.bound
            lit_ch = strip(e.type.to).name
            ok_elems = {'char': {'char', 'signed char', 'unsigned char'}}.get(lit_ch, {lit_ch})
            if strip(p.to).name in ok_elems and (p.bound is None or p.bound >= n):
                out = identity()
                out.is_list = True
                out.to_array = (strip(p.to), p.bound if p.bound is not None else n, p.bound is not None)
                return out
            return None
        if p.bound is None and not elems:
            return None
        if p.bound is not None and len(elems) > p.bound:
            return None
        seqs = []
        for e in elems:
            s = implicit_seq(w, e, p.to)
            if s is None:
                return None
            seqs.append(s)
        if p.bound is not None and p.bound > len(elems):
            if implicit_seq(w, braced(), p.to) is None:
                return None
        out = replace(worst(seqs)) if seqs else identity()
        out.is_list = True
        out.to_array = (strip(p.to), p.bound if p.bound is not None else len(elems), p.bound is not None)
        return out
    T = strip(p)
    # not a class (ICS-15)
    if T.kind != 'class':
        if desig:
            return None
        if not elems:
            return mark_list(identity())
        if len(elems) == 1 and elems[0].braced is None:
            return mark_list(implicit_seq(w, elems[0], p, ctx))
        return None
    info = w.classes.get(T.name)
    if info is None:
        return None
    one_obj = (len(elems) == 1 and elems[0].braced is None and elems[0].type is not None
               and ref_related(w, T, elems[0].type) and not desig)
    # aggregate (ICS-18)
    if info.aggregate:
        if one_obj:
            return mark_list(implicit_seq(w, elems[0], p, ctx))
        flds = list(info.fields)
        if desig:
            names = [n for n, _ in flds]
            if any(d not in names for d in desig):
                return None
            pairs = [(dict(flds)[d], e) for d, e in zip(desig, elems)]
        else:
            if len(elems) > len(flds):
                return None
            pairs = [(t, e) for (n, t), e in zip(flds, elems)]
        for t, e in pairs:
            if implicit_seq(w, e, t) is None:
                return None
        out = ICS('user', aggregate=T.name, third=Std(src=T, dst=T), is_list=True)
        return out
    if desig:
        return None
    # class with constructors (ICS-19, ILC-2, ILC-3)
    ctors = info.ctors
    if not elems and has_default_ctor(info):
        dc = [c for c in ctors if len(c.params) <= c.defaults]
        res = resolve(w, dc, [])
        return list_result(res, T)
    il = [c for c in ctors if c.params and c.params[0].kind == 'ilist' and len(c.params) - 1 <= c.defaults]
    if il:
        res = resolve(w, il, [arg], _list_whole=True)
        if res.status != 'no viable function':
            return list_result(res, T)
    if one_obj:
        return mark_list(implicit_seq(w, elems[0], p, ctx))
    std_only = ()
    if len(elems) == 1 and elems[0].braced is not None:
        std_only = ('inner', T.name)
    res = resolve(w, ctors, list(elems), _inner_std_only=std_only)
    return list_result(res, T)


def has_default_ctor(info):
    if info.default_ctor is not None:
        return info.default_ctor
    return any(len(c.params) <= c.defaults for c in info.ctors)


def list_result(res, T):
    if res.status == 'ok':
        return ICS('user', func=res.fn, third=Std(src=T, dst=T), is_list=True)
    if res.status == 'ambiguous':
        return ICS('ambiguous', is_list=True)
    return None


def mark_list(s):
    if s is None:
        return None
    s = replace(s)
    s.is_list = True
    return s


# ---------------------------------------------------------------- comparing two sequences

def compare_ics(s1, s2):
    """ICS-P5: 1 if s1 is better, -1 if s2 is better, 0 otherwise."""
    k = {'standard': 3, 'user': 2, 'ambiguous': 2, 'ellipsis': 1}
    if s1.static_slot or s2.static_slot:
        # ICS-14, OVL-25: ties with a standard sequence, better than user-defined
        o = s2 if s1.static_slot else s1
        if o.static_slot or o.kind == 'standard':
            return 0
        r = 1 if k[o.kind] < 3 else 0
        return r if s1.static_slot else -r
    # step 1: kind (ICS-21)
    if k[s1.kind] != k[s2.kind]:
        return 1 if k[s1.kind] > k[s2.kind] else -1
    # step 2: list targets (ICS-22 to ICS-24)
    if s1.is_list and s2.is_list:
        if s1.to_ilist != s2.to_ilist:
            return 1 if s1.to_ilist else -1
        if s1.to_array and s2.to_array and s1.to_array[0] == s2.to_array[0]:
            if s1.to_array[1] != s2.to_array[1]:
                return 1 if s1.to_array[1] < s2.to_array[1] else -1
            if s1.to_array[2] != s2.to_array[2]:
                return 1 if s1.to_array[2] else -1
    # step 3: both user-defined (ICS-25)
    if s1.kind in ('user', 'ambiguous'):
        if s1.kind == 'ambiguous' or s2.kind == 'ambiguous':
            return 0
        same = (s1.func is not None and s1.func is s2.func) or \
               (s1.aggregate and s1.aggregate == s2.aggregate)
        if same and s1.third and s2.third:
            return compare_std(s1.third, s2.third)
        return 0
    if s1.kind == 'ellipsis':
        return 0
    return compare_std(s1.std, s2.std)


def compare_std(a, b, w=None, results=False):
    """Step 4 of ICS-P5.  `results` adds ICS-42 to ICS-44 (ICS-37)."""
    # a: proper subsequence (ICS-26)
    sa, sb = a.steps(), b.steps()
    if sa != sb:
        if not sa and sb:
            return 1
        if not sb and sa:
            return -1
        if all(x in sb for x in sa) and len(sa) < len(sb):
            return 1
        if all(x in sa for x in sb) and len(sb) < len(sa):
            return -1
    # b: rank (ICS-27)
    if a.rank != b.rank:
        return 1 if a.rank > b.rank else -1
    # c: same-rank tie-breakers (ICS-32 to ICS-36), ICS-42 to ICS-44
    r = same_rank_tiebreak(a, b, w)
    if r:
        return r
    if results:
        r = result_tiebreak(a, b, w)
        if r:
            return r
    ra, rb = a.ref, b.ref
    # d: rvalue reference to an rvalue (ICS-28)
    if ra and rb and not ra.implicit_obj_noref and not rb.implicit_obj_noref \
            and ra.to_rvalue and rb.to_rvalue and not ra.to_function and ra.rk != rb.rk:
        return 1 if ra.rk == 'r' else -1
    # e: lvalue reference to a function (ICS-29)
    if ra and rb and ra.to_function and rb.to_function and ra.rk != rb.rk:
        return 1 if ra.rk == 'l' else -1
    # f: fewer cv-qualifiers added (ICS-31)
    if a.qual or b.qual:
        if (a.conv, a.conv_dst) == (b.conv, b.conv_dst) and a.dst and b.dst and strip(a.dst) != strip(b.dst):
            if qual_adjust(a.dst, b.dst):
                return 1
            if qual_adjust(b.dst, a.dst):
                return -1
    # g: fewer cv-qualifiers on the referenced type (ICS-30)
    if ra and rb and strip(ra.referenced) == strip(rb.referenced):
        ca, cb = topcv(ra.referenced), topcv(rb.referenced)
        if ca != cb:
            if ca < cb:
                return 1
            if cb < ca:
                return -1
    # h: two direct bindings (ICS-46)
    if ra and rb and ra.direct and rb.direct and strip(ra.referenced) != strip(rb.referenced):
        if qual_adjust(ptr(ra.referenced), ptr(rb.referenced)):
            return 1
        if qual_adjust(ptr(rb.referenced), ptr(ra.referenced)):
            return -1
    return 0


_W = [None]   # the world of the current resolution, for class relations


def _is_base(b, d):
    w = _W[0]
    return w is not None and w.is_base(b, d)


def _cls_of(t):
    """The class named by a class type, a pointer to class, or a reference."""
    if t is None:
        return None
    t = strip(t)
    if t.kind == 'class':
        return t.name
    if t.kind == 'ptr' and strip(t.to).kind == 'class':
        return strip(t.to).name
    return None


def same_rank_tiebreak(a, b, w):
    # ICS-32: pointer or member pointer to bool is worse
    pb_a = a.conv == 'bool' and a.src is not None and strip(a.src).kind in ('ptr', 'mptr', 'array', 'func')
    pb_b = b.conv == 'bool' and b.src is not None and strip(b.src).kind in ('ptr', 'mptr', 'array', 'func')
    if pb_a != pb_b:
        return -1 if pb_a else 1
    same_src = a.src is not None and b.src is not None and strip(a.src) == strip(b.src)
    # ICS-33: base pointer over void*
    if same_src and {a.conv, b.conv} == {'ptr_base', 'ptr_void'}:
        return 1 if a.conv == 'ptr_base' else -1
    # ICS-34: enumeration promotion to the underlying type
    if {a.conv, b.conv} == {'enum_prom_u', 'enum_prom_p'}:
        return 1 if a.conv == 'enum_prom_u' else -1
    # ICS-35: needs extended floating-point types; not modelled
    # ICS-36: nearer base
    if same_src and a.conv == b.conv and a.conv in ('d2b', 'ptr_base'):
        ma, mb = _cls_of(a.conv_dst), _cls_of(b.conv_dst)
        if ma and mb and ma != mb:
            if _is_base(mb, ma):
                return 1
            if _is_base(ma, mb):
                return -1
    if same_src and a.conv == b.conv == 'mptr_base':
        ma, mb = a.conv_dst.name, b.conv_dst.name
        if ma != mb:
            if _is_base(ma, mb):
                return 1
            if _is_base(mb, ma):
                return -1
    return 0


def result_tiebreak(a, b, w):
    """ICS-42 and ICS-43: two results of different types to one target."""
    if a.src is None or b.src is None:
        return 0
    if strip(a.src) == strip(b.src):
        return 0
    ca, cb = _cls_of(a.src), _cls_of(b.src)
    if a.conv in ('d2b', 'ptr_base') and b.conv in ('d2b', 'ptr_base') or \
            (a.ref and b.ref and ca and cb):
        if ca and cb and ca != cb:
            if _is_base(ca, cb):
                return 1
            if _is_base(cb, ca):
                return -1
    if a.conv == b.conv == 'ptr_void' and ca and cb and ca != cb:
        if _is_base(ca, cb):
            return 1
        if _is_base(cb, ca):
            return -1
    if a.conv == b.conv == 'mptr_base':
        na, nb = strip(a.src).name, strip(b.src).name
        if na != nb:
            if _is_base(nb, na):
                return 1
            if _is_base(na, nb):
                return -1
    return 0


# ---------------------------------------------------------------- overload resolution

@dataclass
class Viable:
    fn: Fn
    seqs: list                   # one per argument position (None = takes no part)
    result: Optional[Std] = None # OVL-26: the result's sequence to the target


@dataclass
class Result:
    status: str                  # ok, ambiguous, no viable function
    fn: Optional[Fn] = None
    viable: list = field(default_factory=list)

    def __repr__(self):
        return f'Result({self.status}, {self.fn})'


def resolve(w, cands, args, has_object=False, order=None, more_constrained=None,
            target=None, _list_whole=False, _inner_std_only=()):
    """Choose the best viable function (OVL-P1, OVL-22 to OVL-36).

    args: the arguments.  If has_object, args[0] is the implied object
    argument: a member with an implicit object parameter meets it with that
    parameter, a static member meets it with its object slot (ICS-14), and
    any other candidate takes it as its first parameter.  Without an object,
    the object parameter of a member takes no part (OVL-23).
    target: for an initialization by a conversion function, the type
    initialized (OVL-26, OVL-29); each conversion function candidate then
    needs `ret`.
    """
    prev = _W[0]
    _W[0] = w
    try:
        viable = []
        for f in cands:
            v = viability(w, f, args, has_object, target, _list_whole, _inner_std_only)
            if v is not None:
                viable.append(v)
        if not viable:
            return Result('no viable function')
        best = pick_best(w, viable, target, order, more_constrained)
        if best is None:
            return Result('ambiguous', viable=viable)
        return Result('ok', best.fn, viable=viable)
    finally:
        _W[0] = prev


def viability(w, f, args, has_object, target, list_whole=False, inner_std_only=()):
    if not f.constraints_ok:                                    # OVL-24
        return None
    seqs = []
    rest = list(args)
    if has_object:
        obj = rest.pop(0)
        if f.kind in ('member', 'conv') or (f.kind == 'member'):
            s = object_seq(w, obj, f, f.owner)
            if s is None:
                return None
            seqs.append(s)
        elif f.kind == 'static':
            seqs.append(ICS('standard', std=Std(), static_slot=True))
        else:
            rest.insert(0, obj)
    params = list(f.params)
    n = len(rest)
    if n > len(params) and not f.ellipsis:                     # OVL-22
        return None
    if n < len(params) and len(params) - n > f.defaults:
        return None
    for i, a in enumerate(rest):
        if i < len(params):
            p = params[i]
            ctx = Ctx(std_only=i in f.std_only)
            if inner_std_only and a.braced is not None and len(params) >= 1:
                # ICS-10, last bullet
                cname = inner_std_only[1]
                pt = strip(p.to) if p.kind == 'ref' else strip(p)
                if pt == cls(cname):
                    ctx.std_only = True
            if ctx.std_only and a.braced is not None:
                s = implicit_seq(w, a, p, ctx)
                if s is not None and s.kind != 'standard':
                    s = None
            else:
                s = implicit_seq(w, a, p, ctx)
        else:
            if a.braced is not None:                            # ICS-6
                return None
            if a.overload_set is not None and len([x for x in a.overload_set if not x[1]]) != 1:
                return None
            s = ICS('ellipsis')
        if s is None:
            return None
        seqs.append(s)
    res = None
    if target is not None:
        res = result_seq(w, f, target)
    return Viable(f, seqs, res)


def result_seq(w, f, target):
    """OVL-26: the sequence from the result of f to the target."""
    if f.kind == 'ctor':
        return Std(src=strip(target), dst=strip(target))
    if f.ret is None:
        return None
    ret = f.ret
    if target.kind == 'ref':
        res = ret.to if ret.kind == 'ref' else ret
        cat = ('lvalue' if ret.rk == 'l' else 'xvalue') if ret.kind == 'ref' else 'prvalue'
        s = ref_seq(w, Arg(res, cat), target, Ctx(std_only=True))
        if s is None:
            s = implicit_seq(w, Arg(res, cat), target)
        return s.std if s is not None and s.kind == 'standard' else (s.third if s else None)
    res = ret.to if ret.kind == 'ref' else ret
    cat = ('lvalue' if ret.rk == 'l' else 'xvalue') if ret.kind == 'ref' else 'prvalue'
    return std_conv(w, res, cat, strip(target))


def pick_best(w, viable, target=None, order=None, more_constrained=None):
    """OVL-34: the viable function better than each other one."""
    if len(viable) == 1:
        return viable[0]
    champ = viable[0]
    for v in viable[1:]:
        if better(w, v, champ, target, order, more_constrained):
            champ = v
    for v in viable:
        if v is champ:
            continue
        if not better(w, champ, v, target, order, more_constrained):
            return None
    return champ


def better(w, v1, v2, target=None, order=None, more_constrained=None):
    """OVL-25 with the tie-breakers of OVL-P3."""
    prev = _W[0]
    _W[0] = w
    try:
        any_better = False
        n = min(len(v1.seqs), len(v2.seqs))
        # align from the end when one has an object sequence and the other not
        s1s, s2s = v1.seqs, v2.seqs
        if len(s1s) != len(s2s):
            if len(s1s) > len(s2s):
                s1s = s1s[len(s1s) - len(s2s):]
            else:
                s2s = s2s[len(s2s) - len(s1s):]
        for a, b in zip(s1s, s2s):
            c = compare_ics(a, b)
            if c < 0:
                return False
            if c > 0:
                any_better = True
        if any_better:
            return True
        return tiebreak(w, v1, v2, target, order, more_constrained) > 0
    finally:
        _W[0] = prev


def tiebreak(w, v1, v2, target, order, more_constrained):
    f1, f2 = v1.fn, v2.fn
    # 2a: OVL-26
    if target is not None and v1.result is not None and v2.result is not None:
        c = compare_std(v1.result, v2.result, w, results=True)
        if c:
            return c
    # 2b: OVL-29
    if target is not None and target.kind == 'ref' and target.to.kind == 'func' \
            and f1.kind == 'conv' and f2.kind == 'conv' and f1.ret and f2.ret:
        k1 = f1.ret.rk if f1.ret.kind == 'ref' else ''
        k2 = f2.ret.rk if f2.ret.kind == 'ref' else ''
        if k1 != k2:
            if k1 == target.rk:
                return 1
            if k2 == target.rk:
                return -1
    # 2c: OVL-27
    if f1.template != f2.template:
        return 1 if not f1.template else -1
    if f1.template and f2.template:
        o = order(f1, f2) if order else None
        # 2d: OVL-28
        if o == 'first':
            return 1
        if o == 'second':
            return -1
        # 2e: OVL-41
        if o == 'equal' and more_constrained:
            if more_constrained(f1, f2):
                return 1
            if more_constrained(f2, f1):
                return -1
    else:
        # 2f: OVL-30
        if more_constrained:
            if more_constrained(f1, f2):
                return 1
            if more_constrained(f2, f1):
                return -1
    # 2g: OVL-31
    if f1.kind == f2.kind == 'ctor' and f1.inherited != f2.inherited:
        n = len(v1.seqs)
        if [strip(p) for p in f1.params[:n]] == [strip(p) for p in f2.params[:n]]:
            return 1 if not f1.inherited else -1
    # 2h: OVL-32
    if f1.rewritten != f2.rewritten:
        return 1 if not f1.rewritten else -1
    # 2i: OVL-33
    if f1.rewritten and f2.rewritten and f1.swapped != f2.swapped:
        return 1 if not f1.swapped else -1
    return 0
