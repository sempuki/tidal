"""I2: partial ordering of templates, from Tidal's ORD rules alone.

Blind component, written in phase 1 from specification/templates/ordering.md
(ORD-P1 to ORD-P4, ORD-1 to ORD-16), the matching rules of
templates/deduction.md (DED-P3, DED-10 to DED-13, DED-20, DED-21, DED-36,
DED-37), the subsumption rules of templates/constraints.md (CNS-1 to CNS-4,
CNS-22, CNS-23, CNS-34), TPL-20, TSP-14 and TSP-15. notes.md lists every step
the text did not give.

Entry points:

    compare(F1, F2, ctx)        -> 'first' | 'second' | 'neither'
    compare_partial(P1, P2, primary_name) -> same

'first' means F1 is more specialized than F2.
"""
from dataclasses import dataclass, field, replace
from typing import Optional, Tuple

C = frozenset({'const'})
V = frozenset({'volatile'})
CV = frozenset({'const', 'volatile'})
NOCV = frozenset()


# ---------------------------------------------------------------- types

@dataclass(frozen=True)
class Ty:
    kind: str           # see the constructors below
    name: str = ''
    args: tuple = ()
    cv: frozenset = NOCV
    extra: object = None

    def __repr__(self):
        return show(self)


def fund(n, cv=NOCV):            return Ty('fund', n, cv=cv)
def tp(n, cv=NOCV):              return Ty('tparam', n, cv=cv)        # type parameter (or pack element)
def ptr(t, cv=NOCV):             return Ty('ptr', args=(t,), cv=cv)
def lref(t):                     return Ty('lref', args=(t,))
def rref(t):                     return Ty('rref', args=(t,))
def arr(t, bound=None):          return Ty('arr', args=(t,), extra=bound)   # bound: int, vp(...), or None
def func(ret, params, noexcept=False, cv=NOCV):
    return Ty('func', args=(ret,) + tuple(params), extra=noexcept, cv=cv)
def cls(n, args=(), cv=NOCV):    return Ty('cls', n, tuple(args), cv=cv)   # a class or class template specialization
def ttcls(n, args=(), cv=NOCV):  return Ty('ttcls', n, tuple(args), cv=cv)  # TT<args>, TT a template template parameter
def dep(base, member, cv=NOCV):  return Ty('dep', member, (base,), cv=cv)   # typename base::member, or decltype
def memptr(t, c, cv=NOCV):       return Ty('memptr', args=(t, c), cv=cv)
def expand(pattern):             return Ty('expand', args=(pattern,))      # pattern...
def val(v, vtype='int'):         return Ty('val', str(v), extra=vtype)     # a constant argument
def vp(n):                       return Ty('vparam', n)                    # use of a constant parameter
def tmpl(n):                     return Ty('tmpl', n)                      # a class template name as an argument
def tmplp(n):                    return Ty('tmplparam', n)                 # use of a template template parameter as an argument


def const(t):
    return replace(t, cv=t.cv | C)


def volatile(t):
    return replace(t, cv=t.cv | V)


def show(t):
    q = ''.join(sorted(x[0] for x in t.cv))
    q = ('c' * ('const' in t.cv)) + ('v' * ('volatile' in t.cv))
    pre = {'c': 'const ', 'v': 'volatile ', 'cv': 'const volatile ', '': ''}[q]
    k = t.kind
    if k in ('fund', 'tparam', 'inv'):
        return pre + t.name
    if k == 'ptr':
        return show(t.args[0]) + '*' + (' ' + pre.strip() if pre else '')
    if k == 'lref':
        return show(t.args[0]) + '&'
    if k == 'rref':
        return show(t.args[0]) + '&&'
    if k == 'arr':
        return show(t.args[0]) + '[%s]' % ('' if t.extra is None else (show(t.extra) if isinstance(t.extra, Ty) else t.extra))
    if k == 'func':
        return '%s(%s)%s' % (show(t.args[0]), ', '.join(map(show, t.args[1:])), ' noexcept' if t.extra else '')
    if k in ('cls', 'ttcls'):
        return pre + t.name + ('<%s>' % ', '.join(map(show, t.args)) if t.args else '')
    if k == 'dep':
        return pre + 'typename %s::%s' % (show(t.args[0]), t.name)
    if k == 'memptr':
        return '%s %s::*' % (show(t.args[0]), show(t.args[1]))
    if k == 'expand':
        return show(t.args[0]) + '...'
    return t.name


# ---------------------------------------------------------------- declarations

@dataclass(frozen=True)
class TP:
    name: str
    kind: str = 'type'          # 'type', 'value', 'template'
    pack: bool = False
    vtype: object = None        # for a constant parameter: 'int', 'auto', 'auto*', ... (or a Ty)


@dataclass
class Param:
    ty: Ty
    default: bool = False


@dataclass
class FT:
    """A function template declaration."""
    name: str
    tparams: Tuple[TP, ...]
    params: Tuple[Param, ...] = ()
    ret: Ty = field(default_factory=lambda: fund('void'))
    member: bool = False        # a member with an implicit object parameter
    cls: Optional[Ty] = None    # the class of a member
    refq: Optional[str] = None  # None, '&', '&&'
    mcv: frozenset = NOCV       # cv-qualifiers of a member
    constraints: object = None  # normal form: None, atom, ('and', a, b), ('or', a, b)
    ellipsis: bool = False
    noexcept: bool = False

    def ends_in_pack(self):
        return bool(self.params) and self.params[-1].ty.kind == 'expand'


@dataclass
class PS:
    """A partial specialization (or the primary template, with primary=True)."""
    tparams: Tuple[TP, ...]
    args: Tuple[Ty, ...]
    constraints: object = None
    primary: bool = False


@dataclass
class Ctx:
    kind: str = 'call'          # 'call', 'conversion', 'address', 'declaration', 'placement'
    nargs: Optional[int] = None  # call: number of arguments, the object operand included for a member
    swapped: Tuple[bool, bool] = (False, False)  # ORD-7: is F1 / F2 a swapped rewritten candidate


# ---------------------------------------------------------------- step 0: the types that take part (ORD-P1, ORD-5 to ORD-9)

def adjust_param(t):
    """The function type keeps a parameter without its top-level cv (choice, notes.md N10)."""
    return replace(t, cv=NOCV) if t.kind not in ('lref', 'rref', 'expand') else t


def call_types(F, nargs):
    """ORD-5: the parameters for which the call has arguments; ORD-4: a pack once."""
    explicit = nargs - (1 if F.member else 0)
    out = []
    used = 0
    for p in F.params:
        if p.ty.kind == 'expand':
            if explicit > used:         # the pack has arguments (notes.md N4)
                out.append(p.ty)
            break
        if used >= explicit:
            break                        # default argument used: no part
        out.append(p.ty)
        used += 1
    return out


def object_param(F, kind):
    c = replace(F.cls, cv=F.cls.cv | F.mcv)
    return rref(c) if kind == 'r' else lref(c)


def types_taking_part(F1, F2, ctx):
    k = ctx.kind
    if k == 'conversion':
        return [F1.ret], [F2.ret]
    if k in ('address', 'declaration', 'placement'):
        def ft(F):
            return [func(F.ret, [adjust_param(p.ty) for p in F.params], F.noexcept)]
        return ft(F1), ft(F2)
    # a call
    L = [call_types(F1, ctx.nargs), call_types(F2, ctx.nargs)]
    Fs = [F1, F2]
    # ORD-7 order: object parameter first, then reversal. The other template's
    # "first parameter" for ORD-6 is read from its list as it takes part
    # without the object parameter, after its own reversal (notes.md N6).
    for i in (0, 1):
        if ctx.swapped[i] and not Fs[i].member and len(L[i]) >= 2:
            L[i] = [L[i][1], L[i][0]] + L[i][2:]
    obj = [None, None]
    for i in (0, 1):
        F, other = Fs[i], Fs[1 - i]
        if not F.member:
            continue
        if F.refq == '&&':
            kind = 'r'
        elif F.refq == '&':
            kind = 'l'
        else:
            if other.member:
                # both members: take the kind of the other's object parameter (ORD-P1, notes.md N5)
                kind = 'r' if other.refq == '&&' else 'l'
            else:
                first = L[1 - i][0] if L[1 - i] else None
                kind = 'r' if (first is not None and first.kind == 'rref') else 'l'
        obj[i] = object_param(F, kind)
    for i in (0, 1):
        if obj[i] is not None:
            L[i] = [obj[i]] + L[i]
            if ctx.swapped[i] and len(L[i]) >= 2:
                L[i] = [L[i][1], L[i][0]] + L[i][2:]
    return L[0], L[1]


# ---------------------------------------------------------------- step 1: invent (ORD-1)

def invent(tparams, tag):
    m = {}
    for p in tparams:
        n = '%s#%s' % (p.name, tag)
        if p.kind == 'type':
            m[p.name] = Ty('inv', n, extra=('pack' if p.pack else None))
        elif p.kind == 'value':
            m[p.name] = Ty('invval', n, extra=p.vtype)
        else:
            m[p.name] = Ty('invtmpl', n)
    return m


def subst(t, m):
    k = t.kind
    if k == 'tparam' and t.name in m:
        r = m[t.name]
        return replace(r, cv=r.cv | t.cv)
    if k == 'vparam' and t.name in m:
        return m[t.name]
    if k == 'tmplparam' and t.name in m:
        return Ty('tmpl', m[t.name].name)
    if k == 'ttcls' and t.name in m:
        return Ty('cls', m[t.name].name, tuple(subst(a, m) for a in t.args), cv=t.cv)
    if k == 'arr':
        b = t.extra
        if isinstance(b, Ty):
            b = subst(b, m)
        return Ty('arr', args=(subst(t.args[0], m),), extra=b, cv=t.cv)
    if t.args:
        return replace(t, args=tuple(subst(a, m) for a in t.args))
    return t


# ---------------------------------------------------------------- step 2: strip (ORD-2)

def strip(t):
    """Returns (stripped type, reference kind or None, cv of the referred type)."""
    if t.kind == 'expand':
        s, rk, q = strip(t.args[0])
        return expand(s), rk, q
    rk = None
    if t.kind in ('lref', 'rref'):
        rk = 'l' if t.kind == 'lref' else 'r'
        t = t.args[0]
    return replace(t, cv=NOCV), rk, t.cv


# ---------------------------------------------------------------- step 3: deduce (ORD-3, ORD-4, ORD-16, DED-P3)

class Fail(Exception):
    pass


class Env:
    def __init__(self, tparams):
        self.tp = {p.name: p for p in tparams}
        self.vals = {}
        self.packs = {}

    def bind(self, name, v, in_pack):
        p = self.tp[name]
        if p.pack:
            if not in_pack:
                raise Fail('pack %s outside an expansion' % name)
            self.packs.setdefault(name, []).append(v)
            return
        if name in self.vals and self.vals[name] != v:
            raise Fail('%s is %s and %s' % (name, self.vals[name], v))
        self.vals[name] = v

    def merge(self, other):
        for n, v in other.vals.items():
            if n in self.vals and self.vals[n] != v:
                raise Fail('%s is %s and %s (DED-21)' % (n, self.vals[n], v))
            self.vals[n] = v
        for n, v in other.packs.items():
            self.packs.setdefault(n, []).extend(v)   # packs never conflict (notes.md N9)


def vtype_fits(ptype, atype):
    """DED-12 for a constant parameter of F2 against an invented or given value (notes.md N12)."""
    if ptype in (None, 'auto'):
        return True
    if ptype == 'auto*':
        return isinstance(atype, str) and atype.endswith('*')
    return ptype == atype


def match(P, A, env, in_pack=False):
    """Matches F2's type P against F1's invented type A, exactly (ORD-3)."""
    k = P.kind
    if k == 'dep':
        return              # ORD-16: no value, not checked
    if k == 'tparam' and P.name in env.tp:
        if A.kind == 'expand':
            raise Fail('an invented pack against %s (ORD-4)' % show(P))
        if not P.cv <= A.cv:
            raise Fail('%s needs more cv than %s' % (show(P), show(A)))
        env.bind(P.name, replace(A, cv=A.cv - P.cv), in_pack)
        return
    if k == 'vparam' and P.name in env.tp:
        if A.kind not in ('val', 'invval'):
            raise Fail('value parameter against %s' % show(A))
        if not vtype_fits(env.tp[P.name].vtype, A.extra):
            raise Fail('value type %s does not fit %s (DED-12)' % (A.extra, env.tp[P.name].vtype))
        env.bind(P.name, A, in_pack)
        return
    if k == 'tmplparam' and P.name in env.tp:
        if A.kind != 'tmpl':
            raise Fail('template parameter against %s' % show(A))
        env.bind(P.name, A, in_pack)
        return
    if A.kind == 'expand':
        raise Fail('an invented pack against %s (ORD-4)' % show(P))
    if k == 'ttcls' and P.name in env.tp:
        if A.kind != 'cls' or P.cv != A.cv:
            raise Fail('%s against %s' % (show(P), show(A)))
        env.bind(P.name, Ty('tmpl', A.name), in_pack)
        match_list(P.args, A.args, env, in_pack)
        return
    if P.cv != A.cv or k != A.kind:
        raise Fail('%s against %s' % (show(P), show(A)))
    if k in ('fund', 'inv', 'invval', 'invtmpl', 'tmpl', 'val'):
        if P.name != A.name:
            raise Fail('%s against %s' % (show(P), show(A)))
        return
    if k == 'dep':
        return
    if k in ('ptr', 'lref', 'rref', 'memptr'):
        for p, a in zip(P.args, A.args):
            match(p, a, env, in_pack)
        return
    if k == 'arr':
        match(P.args[0], A.args[0], env, in_pack)
        pb, ab = P.extra, A.extra
        if isinstance(pb, Ty) and pb.kind == 'vparam' and pb.name in env.tp:
            if ab is None:
                raise Fail('bound against no bound')
            av = ab if isinstance(ab, Ty) else val(ab, 'size_t')
            env.bind(pb.name, av, in_pack)
        elif pb != ab:
            raise Fail('bounds %s and %s' % (pb, ab))
        return
    if k == 'func':
        match(P.args[0], A.args[0], env, in_pack)
        match_list(P.args[1:], A.args[1:], env, in_pack)
        pn, an = P.extra, A.extra
        if isinstance(pn, Ty) and pn.kind == 'vparam' and pn.name in env.tp:
            env.bind(pn.name, an if isinstance(an, Ty) else val(an, 'bool'), in_pack)
        elif pn != an:
            raise Fail('noexcept differs')
        return
    if k == 'cls':
        if P.name != A.name:
            raise Fail('%s against %s' % (show(P), show(A)))
        match_list(P.args, A.args, env, in_pack)
        return
    raise Fail('no form for %s' % show(P))


def match_list(Ps, As, env, in_pack):
    """DED-37 with ORD-4: an expansion at the end of P takes the rest of A."""
    i = 0
    for j, p in enumerate(Ps):
        if p.kind == 'expand':
            if j != len(Ps) - 1:
                return      # a pack before the end: non-deduced (DED-20 by analogy)
            for a in As[i:]:
                if a.kind == 'expand':
                    match(p.args[0], a.args[0], env, True)
                else:
                    match(p.args[0], a, env, True)
            return
        if i >= len(As):
            raise Fail('too few arguments')
        if As[i].kind == 'expand':
            raise Fail('an invented pack against %s (ORD-4)' % show(p))
        match(p, As[i], env, in_pack)
        i += 1
    if i < len(As):
        raise Fail('too many arguments')


# ---------------------------------------------------------------- pairing

def make_pairs(L1, L2):
    """Pairs by position. An expansion pairs with each remaining entry of the
    other list. Entries with nothing against them take no part (ORD-4)."""
    pairs = []
    i = j = 0
    while i < len(L1) and j < len(L2):
        a, b = L1[i], L2[j]
        if a.kind == 'expand' and b.kind == 'expand':
            pairs.append((i, j))
            break
        if b.kind == 'expand':
            for ii in range(i, len(L1)):
                pairs.append((ii, j))
            break
        if a.kind == 'expand':
            for jj in range(j, len(L2)):
                pairs.append((i, jj))
            break
        pairs.append((i, j))
        i += 1
        j += 1
    return pairs


def deduce_pair(P, A, tparams):
    env = Env(tparams)
    if P.kind == 'expand':
        if A.kind == 'expand':
            match(P.args[0], A.args[0], env, True)
        else:
            match(P.args[0], A, env, True)
    else:
        match(P, A, env)
    return env


# ---------------------------------------------------------------- steps 1 to 6 (ORD-P2)

def direction(src_types, src_tparams, dst_types, dst_tparams, pairs, flip, trace):
    """Deduces dst's parameters from src's invented types. Returns a list of
    per-pair success flags and the combination result."""
    m = invent(src_tparams, 'src')
    inv = [strip(subst(t, m)) for t in src_types]
    dst = [strip(t) for t in dst_types]
    ok = []
    total = Env(dst_tparams)
    combined = True
    for (i, j) in pairs:
        si, di = (i, j) if not flip else (j, i)
        A = inv[si][0]
        P = dst[di][0]
        try:
            env = deduce_pair(P, A, dst_tparams)
            ok.append(True)
            try:
                total.merge(env)
            except Fail as e:
                combined = False
                trace.append('combine fails: %s' % e)
        except Fail as e:
            ok.append(False)
            trace.append('pair %s <- %s fails: %s' % (show(P), show(A), e))
    return ok, combined, inv, dst


def at_least(L1, T1, L2, T2, trace):
    """Steps 1 to 5 both ways. Returns (F1 >= F2, F2 >= F1)."""
    pairs = make_pairs(L1, L2)
    trace.append('pairs: %s' % [(show(L1[i]), show(L2[j])) for i, j in pairs])
    ok12, comb12, _, _ = direction(L1, T1, L2, T2, pairs, False, trace)
    ok21, comb21, _, _ = direction(L2, T2, L1, T1, pairs, True, trace)
    s1 = [strip(t) for t in L1]
    s2 = [strip(t) for t in L2]
    keep12 = list(ok12)
    keep21 = list(ok21)
    for n, (i, j) in enumerate(pairs):
        _, r1, q1 = s1[i]
        _, r2, q2 = s2[j]
        if r1 is None or r2 is None or not (ok12[n] and ok21[n]):
            continue
        # ORD-10
        if r1 != r2:
            if r1 == 'r':
                keep12[n] = False
            else:
                keep21[n] = False
            trace.append('ORD-10 on pair %d' % n)
        # ORD-11
        if q1 > q2:
            keep21[n] = False
            trace.append('ORD-11 on pair %d: F1 more qualified' % n)
        elif q2 > q1:
            keep12[n] = False
            trace.append('ORD-11 on pair %d: F2 more qualified' % n)
    ge12 = comb12 and all(keep12)
    ge21 = comb21 and all(keep21)
    trace.append('F1>=F2: %s, F2>=F1: %s' % (ge12, ge21))
    return ge12, ge21


# ---------------------------------------------------------------- constraints (CNS-22, CNS-23, CNS-34)

def rename(t, m):
    """Renames template parameters (CNS-34 correspondence)."""
    if isinstance(t, Ty):
        n = m.get(t.name, t.name) if t.kind in ('tparam', 'vparam', 'tmplparam', 'ttcls') else t.name
        ex = rename(t.extra, m) if isinstance(t.extra, Ty) else t.extra
        return Ty(t.kind, n, tuple(rename(a, m) for a in t.args), t.cv, ex)
    if isinstance(t, tuple):
        return tuple(rename(x, m) for x in t)
    if isinstance(t, str):
        return m.get(t, t)
    return t


def dnf(f):
    if f is None:
        return [frozenset()]
    if isinstance(f, tuple) and f and f[0] == 'and':
        return [a | b for a in dnf(f[1]) for b in dnf(f[2])]
    if isinstance(f, tuple) and f and f[0] == 'or':
        return dnf(f[1]) + dnf(f[2])
    return [frozenset([f])]


def cnf(f):
    if isinstance(f, tuple) and f and f[0] == 'and':
        return cnf(f[1]) + cnf(f[2])
    if isinstance(f, tuple) and f and f[0] == 'or':
        return [a | b for a in cnf(f[1]) for b in cnf(f[2])]
    return [frozenset([f])]


def subsumes(P, Q):
    return all(c & d for c in dnf(P) for d in cnf(Q))


def at_least_constrained(c1, c2):
    if c2 is None:
        return True
    if c1 is None:
        return False
    return subsumes(c1, c2)


def more_constrained(c1, c2):
    a, b = at_least_constrained(c1, c2), at_least_constrained(c2, c1)
    if a and not b:
        return 'first'
    if b and not a:
        return 'second'
    return 'neither'


def heads_equivalent(T1, T2, m):
    if len(T1) != len(T2):
        return False
    for a, b in zip(T1, T2):
        if a.kind != b.kind or a.pack != b.pack:
            return False
        if a.kind == 'value' and rename(b.vtype, m) != a.vtype:
            return False
    return True


def correspondence(T1, T2):
    return {b.name: a.name for a, b in zip(T1, T2)}


def ord13_conditions(F1, F2, ctx, trace):
    m = correspondence(F1.tparams, F2.tparams)
    if len(F1.params) != len(F2.params):
        trace.append('ORD-13: parameter lists differ in length')
        return False
    for p1, p2 in zip(F1.params, F2.params):
        if adjust_param(p1.ty) != rename(adjust_param(p2.ty), m):
            trace.append('ORD-13: parameter types differ: %s, %s' % (show(p1.ty), show(p2.ty)))
            return False
    if ctx.kind == 'conversion' and F1.ret != rename(F2.ret, m):
        trace.append('ORD-13: result types differ')
        return False
    if not heads_equivalent(F1.tparams, F2.tparams, m):
        trace.append('ORD-13: heads not equivalent')
        return False
    return True


# ---------------------------------------------------------------- entry points

def compare(F1, F2, ctx=None, trace=None):
    ctx = ctx or Ctx()
    trace = [] if trace is None else trace
    L1, L2 = types_taking_part(F1, F2, ctx)
    trace.append('F1 types: %s; F2 types: %s' % (L1, L2))
    ge12, ge21 = at_least(L1, F1.tparams, L2, F2.tparams, trace)
    if ge12 and not ge21:
        return 'first'
    if ge21 and not ge12:
        return 'second'
    if not ge12 and not ge21:
        return 'neither'
    # 6c: ORD-12 on the declared parameter lists
    p1, p2 = F1.ends_in_pack(), F2.ends_in_pack()
    if p1 != p2:
        packF, other = (F1, F2) if p1 else (F2, F1)
        pos = len(packF.params) - 1
        if len(other.params) <= pos:
            trace.append('ORD-12 decides')
            return 'second' if p1 else 'first'
    # 6d: ORD-13
    if ord13_conditions(F1, F2, ctx, trace):
        m = correspondence(F1.tparams, F2.tparams)
        r = more_constrained(F1.constraints, rename(F2.constraints, m))
        trace.append('ORD-13: %s' % r)
        return r
    return 'neither'


def compare_partial(P1, P2, primary_name, trace=None):
    """ORD-14: each is a function template with one parameter, the class
    template specialization with its argument list."""
    trace = [] if trace is None else trace
    L1 = [cls(primary_name, P1.args)]
    L2 = [cls(primary_name, P2.args)]
    ge12, ge21 = at_least(L1, P1.tparams, L2, P2.tparams, trace)
    if ge12 and not ge21:
        return 'first'
    if ge21 and not ge12:
        return 'second'
    if not ge12 and not ge21:
        return 'neither'
    m = correspondence(P1.tparams, P2.tparams)
    if heads_equivalent(P1.tparams, P2.tparams, m) and tuple(P1.args) == rename(tuple(P2.args), m):
        r = more_constrained(P1.constraints, rename(P2.constraints, m))
        trace.append('ORD-14 constraints: %s' % r)
        return r
    trace.append('ORD-14: patterns differ, constraints not compared')
    return 'neither'


def primary(tparams):
    """The primary template as a partial specialization (ORD-14)."""
    args = []
    for p in tparams:
        a = tp(p.name) if p.kind == 'type' else vp(p.name) if p.kind == 'value' else tmplp(p.name)
        args.append(expand(a) if p.pack else a)
    return PS(tuple(tparams), tuple(args), primary=True)


if __name__ == '__main__':
    T = (TP('T'),)
    f1 = FT('pick', T, (Param(tp('T')),))
    f2 = FT('pick', T, (Param(ptr(tp('T'))),))
    f3 = FT('pick', T, (Param(ptr(const(tp('T')))),))
    print('T vs T*', compare(f1, f2, Ctx(nargs=1)))
    print('T* vs const T*', compare(f2, f3, Ctx(nargs=1)))
    take1 = FT('take', T, (Param(tp('T')),))
    take2 = FT('take', T, (Param(lref(const(tp('T')))),))
    print('T vs const T&', compare(take1, take2, Ctx(nargs=1)))
    b1 = FT('bind', T, (Param(lref(tp('T'))),))
    b2 = FT('bind', T, (Param(rref(tp('T'))),))
    print('T& vs T&&', compare(b1, b2, Ctx(nargs=1)))
    for r1, r2, want in [(lref(tp('T')), rref(tp('T')), 'first'),
                         (lref(tp('T')), lref(const(tp('T'))), 'second'),
                         (lref(const(tp('T'))), rref(tp('T')), 'first'),
                         (rref(tp('T')), rref(const(tp('T'))), 'second'),
                         (lref(tp('T')), rref(const(tp('T'))), 'neither')]:
        a = FT('conv', T, ret=r1, member=True, cls=cls('Source'))
        b = FT('conv', T, ret=r2, member=True, cls=cls('Source'))
        print('conv', r1, r2, compare(a, b, Ctx('conversion')), 'page:', want)
    TU = (TP('T'), TP('U'))
    prim = primary(TU)
    A = PS((TP('T'),), (tp('T'), tp('T')))
    B = PS(TU, (ptr(tp('T')), tp('U')))
    Cc = PS((TP('T'),), (ptr(tp('T')), ptr(tp('T'))))
    print('C vs A', compare_partial(Cc, A, 'Pair'), 'C vs B', compare_partial(Cc, B, 'Pair'),
          'A vs primary', compare_partial(A, prim, 'Pair'))
