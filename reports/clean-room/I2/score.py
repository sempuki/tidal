"""I2 scoring. Each ordering question below is a hand-written description of
two declarations from a test, written after reading the test. The script
does not parse C++. ordering.py is the blind component and is not changed.

A check is one question "which of these two templates is more specialized"
that the test's expected outcome decides. Calls where only one template is
viable, or where the conversion sequences decide, are not checks.
"""
import sys
from collections import OrderedDict
from ordering import *

T = (TP('T'),)
TU = (TP('T'), TP('U'))


def ft(tparams, *params, **kw):
    ps = []
    for p in params:
        ps.append(p if isinstance(p, Param) else Param(p))
    return FT('f', tuple(tparams), tuple(ps), **kw)


def d(t):          # a parameter with a default argument
    return Param(t, default=True)


def call(n, swapped=(False, False)):
    return Ctx('call', nargs=n, swapped=swapped)


tT, tU = tp('T'), tp('U')
checks = OrderedDict()


def add(file, label, f1, f2, ctx, want, partial=None):
    checks.setdefault(file, []).append((label, f1, f2, ctx, want, partial))


O = 'templates/ordering/'
# more-specialized.cpp
add(O + 'more-specialized.cpp', 'f(p): T vs T*', ft(T, tT), ft(T, ptr(tT)), call(1), 'second')
add(O + 'more-specialized.cpp', 'f(q): T vs const T*', ft(T, tT), ft(T, ptr(const(tT))), call(1), 'second')
add(O + 'more-specialized.cpp', 'f(q): T* vs const T*', ft(T, ptr(tT)), ft(T, ptr(const(tT))), call(1), 'second')
# repeated-parameter.cpp
add(O + 'repeated-parameter.cpp', 'pair(1,2): (T,T) vs (T,U)', ft(T, tT, tT), ft(TU, tT, tU), call(2), 'first')
# neither.cpp
add(O + 'neither.cpp', 'put(T,int) vs put(int,T)', ft(T, tT, fund('int')), ft(T, fund('int'), tT), call(2), 'neither')
add(O + 'reference-and-cv-removed.cpp', 'T vs const T&', ft(T, tT), ft(T, lref(const(tT))), call(1), 'neither')
add(O + 'reference-removed-forwarding.cpp', 'T vs T&&', ft(T, tT), ft(T, rref(tT)), call(1), 'neither')
add(O + 'unused-parameter.cpp', '<T,U=void> f(T) vs f(T*)', ft(TU, tT), ft(T, ptr(tT)), call(1), 'second')
RT = (TP('R'), TP('T'))
add(O + 'unused-parameter-explicit.cpp', '<R,T> f(T) vs <R,T> f(T*)', ft(RT, tT), ft(RT, ptr(tT)), call(1), 'second')
Ts = (TP('Ts', pack=True),)
TTs = (TP('T'), TP('Ts', pack=True))
add(O + 'pack-against-single.cpp', 'f(1): T vs Ts...', ft(T, tT), ft(Ts, expand(tp('Ts'))), call(1), 'first')
add(O + 'pack-against-single.cpp', 'g(1): (T,Ts...) vs Ts...', ft(TTs, tT, expand(tp('Ts'))), ft(Ts, expand(tp('Ts'))), call(1), 'first')
add(O + 'pack-against-single.cpp', 'g(1,2,3): (T,Ts...) vs Ts...', ft(TTs, tT, expand(tp('Ts'))), ft(Ts, expand(tp('Ts'))), call(3), 'first')
A_ = (TP('A', pack=True),)
TA = (TP('T'), TP('A', pack=True))
one = ft(A_, cls('List', [expand(tp('A'))]))
two = ft(TA, cls('List', [tT, expand(tp('A'))]))
three = ft(TA, cls('List', [tT, expand(lref(tp('A')))]))
P = O + 'pack-class-arguments.cpp'
add(P, 'List<int,float>: One vs Two', one, two, call(1), 'second')
add(P, 'List<int,float&>: One vs Three', one, three, call(1), 'second')
add(P, 'List<int,float&>: Two vs Three', two, three, call(1), 'second')
add(P, 'List<int>: One vs Three', one, three, call(1), 'second')
add(P, 'List<int>: Two vs Three', two, three, call(1), 'second')
idT = dep(cls('Id', [tT]), 'type')
add(O + 'nondeduced-position.cpp', 'f: (T, Id<T>::type) vs (T,T)', ft(T, tT, idT), ft(T, tT, tT), call(2), 'second')
add(O + 'nondeduced-position.cpp', 'g: (T, Id<T>::type) vs (T,int)', ft(T, tT, idT), ft(T, tT, fund('int')), call(2), 'second')
add(O + 'nondeduced-position-pointer.cpp', '(T*, Id<T>::type*) vs (T*,T*)', ft(T, ptr(tT), ptr(idT)), ft(T, ptr(tT), ptr(tT)), call(2), 'second')
add(O + 'call-arguments-only.cpp', 'f(T*, int=0) vs f(T)', ft(T, ptr(tT), d(fund('int'))), ft(T, tT), call(1), 'first')
add(O + 'call-arguments-only.cpp', 'g(T*, ...) vs g(T)', ft(T, ptr(tT), ellipsis=True), ft(T, tT), call(1), 'first')
add(O + 'call-defaults-tie.cpp', 'f(T, int=0) vs f(T)', ft(T, tT, d(fund('int'))), ft(T, tT), call(1), 'neither')
X = cls('X')
add(O + 'member-object.cpp', 'X::+(T*) vs +(T&, U)', ft(T, ptr(tT), member=True, cls=X), ft(TU, lref(tT), tU), call(2), 'first')
Y = cls('Y')
add(O + 'member-object.cpp', 'Y::+(T*)&& vs +(T&&, U)', ft(T, ptr(tT), member=True, cls=Y, refq='&&'), ft(TU, rref(tT), tU), call(2), 'first')
add(O + 'member-object-rvalue.cpp', 'X::+(T) vs +(X&&, T)', ft(T, tT, member=True, cls=X), ft(T, rref(X), tT), call(2), 'neither')
add(O + 'reversed-member-object.cpp', 'swapped X::==(T* const&) const vs ==(const T&, const X&)',
    ft(T, lref(ptr(tT, cv=C)), member=True, cls=X, mcv=C), ft(T, lref(const(tT)), lref(const(X))),
    call(2, (True, False)), 'first')
W = lambda t: cls('W', [t])
add(O + 'swapped-member-candidates-ordered.cpp', 'swapped X::==(const W<T>&) vs swapped X::==(const W<T*>&)',
    ft(T, lref(const(W(tT))), member=True, cls=X, mcv=C), ft(T, lref(const(W(ptr(tT)))), member=True, cls=X, mcv=C),
    call(2, (True, True)), 'second')
add(O + 'reversed-candidate.cpp', '==(const X&, const T&) vs swapped ==(T* const&, const X&)',
    ft(T, lref(const(X)), lref(const(tT))), ft(T, lref(ptr(tT, cv=C)), lref(const(X))),
    call(2, (False, True)), 'second')
Vt = lambda t: cls('V', [t])
add(O + 'swapped-candidates-ordered.cpp', 'swapped (const V<U>&, const W<T>&) vs swapped (const V<T*>&, const W<T*>&)',
    ft(TU, lref(const(Vt(tU))), lref(const(W(tT)))), ft(T, lref(const(Vt(ptr(tT)))), lref(const(W(ptr(tT))))),
    call(2, (True, True)), 'second')
S = cls('Source')


def conv(ret, **kw):
    return FT('conv', T, (), ret=ret, member=True, cls=S, mcv=C, **kw)


add(O + 'conversion-result.cpp', 'operator T vs operator T*', conv(tT), conv(ptr(tT)), Ctx('conversion'), 'second')
R = O + 'reference-kinds-conversion.cpp'
add(R, 'Lvalue: T& vs T&&', conv(lref(tT)), conv(rref(tT)), Ctx('conversion'), 'first')
add(R, 'Qualified: T& vs const T&', conv(lref(tT)), conv(lref(const(tT))), Ctx('conversion'), 'second')
add(R, 'ConstLvalue: const T& vs T&&', conv(lref(const(tT))), conv(rref(tT)), Ctx('conversion'), 'first')
add(R, 'ConstRvalue: T&& vs const T&&', conv(rref(tT)), conv(rref(const(tT))), Ctx('conversion'), 'second')
add(O + 'reference-kind-and-qualifiers.cpp', 'T& vs const T&&', conv(lref(tT)), conv(rref(const(tT))), Ctx('conversion'), 'neither')
add(O + 'address-function-type.cpp', 'make: T*(int) vs T(int)', ft(T, fund('int'), ret=ptr(tT)), ft(T, fund('int'), ret=tT), Ctx('address'), 'first')
add(O + 'address-function-type.cpp', 'f(T) vs f(T*) to int(*)(int*)', ft(T, tT, ret=fund('int')), ft(T, ptr(tT), ret=fund('int')), Ctx('address'), 'second')
add(O + 'lvalue-over-rvalue-reference.cpp', 'T& vs T&&', ft(T, lref(tT)), ft(T, rref(tT)), call(1), 'first')
add(O + 'reference-before-variadic.cpp', '(T&, Ts...) vs (T&&)', ft(TTs, lref(tT), expand(tp('Ts'))), ft(T, rref(tT)), call(1), 'first')
add(O + 'reference-pairs-disagree.cpp', '(T&, U&&) vs (T&&, U&)', ft(TU, lref(tT), rref(tU)), ft(TU, rref(tT), lref(tU)), call(2), 'neither')
add(O + 'reference-pairs-one-way.cpp', '(T&&, U*) vs (T&, U)', ft(TU, rref(tT), ptr(tU)), ft(TU, lref(tT), tU), call(2), 'neither')
add(O + 'more-qualified-reference.cpp', 'T& vs const T&', ft(T, lref(tT)), ft(T, lref(const(tT))), call(1), 'second')
add(O + 'non-variadic.cpp', 'f(1): T vs (T, Ts...)', ft(T, tT), ft(TTs, tT, expand(tp('Ts'))), call(1), 'first')
TUp = (TP('T'), TP('U', pack=True))
add(O + 'pack-default-parameter.cpp', '(T, T=T()) vs (T, U...)', ft(T, tT, d(tT)), ft(TUp, tT, expand(tU)), call(1), 'neither')
# partial specializations
pA = PS((TP('T'),), (tT, tT))
pB = PS(TU, (ptr(tT), tU))
pC = PS((TP('T'),), (ptr(tT), ptr(tT)))
add(O + 'partial-specializations.cpp', 'Pair<int*,int*>: C vs A', None, None, None, 'first', (pC, pA, 'Pair'))
add(O + 'partial-specializations.cpp', 'Pair<int*,int*>: C vs B', None, None, None, 'first', (pC, pB, 'Pair'))
add(O + 'partial-specializations.cpp', 'kind<int* const*>: T* vs T* const*', None, None, None, 'second',
    (PS(T, (ptr(tT),)), PS(T, (ptr(ptr(tT, cv=C)),)), 'kind'))
add(O + 'partial-specializations-placeholder.cpp', 'Kind<&np>: auto* P vs auto** P', None, None, None, 'second',
    (PS((TP('P', 'value', vtype='auto*'),), (vp('P'),)), PS((TP('P', 'value', vtype='auto**'),), (vp('P'),)), 'Kind'))
big = ('sizeof>1', 'T')
add(O + 'constraints-conversion.cpp', 'T* vs T* requires', conv(ptr(tT)), conv(ptr(tT), constraints=big), Ctx('conversion'), 'second')
add(O + 'constraints-conversion-results-differ.cpp', 'T* vs T* const requires', conv(ptr(tT)), conv(ptr(tT, cv=C), constraints=big), Ctx('conversion'), 'neither')
add(O + 'constraints-default-parameter.cpp', 'f(T, int=0) requires vs f(T)', ft(T, tT, d(fund('int')), constraints=big), ft(T, tT), call(1), 'neither')
TUd = (TP('T'), TP('U'))
add(O + 'constraints-different-heads.cpp', '<T> f(T) vs <T, U=int> requires f(T)', ft(T, tT), ft(TUd, tT, constraints=big), call(1), 'neither')
add(O + 'constraints-ellipsis.cpp', 'pick(T) requires vs pick(T, ...)', ft(T, tT, constraints=big), ft(T, tT, ellipsis=True), call(1), 'first')
add(O + 'constraints-partial.cpp', 'Kind<int*>: T* vs T* requires', None, None, None, 'second',
    (PS(T, (ptr(tT),)), PS(T, (ptr(tT),), constraints=big), 'Kind'))
weak = ('sizeof>0', 'T')
strong = ('and', ('sizeof>0', 'T'), ('sizeof>1', 'T'))
add(O + 'constraints-placeholder-parameter.cpp', 'f(Strong auto) vs <Weak T> f(T)',
    ft((TP('auto1'),), tp('auto1'), constraints=('and', ('sizeof>0', 'auto1'), ('sizeof>1', 'auto1'))),
    ft(T, tT, constraints=weak), call(1), 'first')
add(O + 'constraints-type-constraint.cpp', '<Strong T> f(T) vs <Weak T> f(T)', ft(T, tT, constraints=strong), ft(T, tT, constraints=weak), call(1), 'first')
DD = 'templates/deduction/'
dv = ptr(fund('void'))
add(DD + 'declaration-specialization.cpp', 'template<> which(int*): which(T) vs which(T*)', ft(T, tT, ret=fund('int')), ft(T, ptr(tT), ret=fund('int')), Ctx('declaration'), 'second')
add(DD + 'declaration-specialization.cpp', 'template which(long*): which(T) vs which(T*)', ft(T, tT, ret=fund('int')), ft(T, ptr(tT), ret=fund('int')), Ctx('declaration'), 'second')
add(DD + 'placement-deallocation-ordered.cpp', 'delete(void*, T*) vs delete(void*, T)', ft(T, dv, ptr(tT)), ft(T, dv, tT), Ctx('placement'), 'first')
add(DD + 'placement-deallocation-ordered-instantiation.cpp', 'delete(void*, T*) vs delete(void*, T)', ft(T, dv, ptr(tT)), ft(T, dv, tT), Ctx('placement'), 'first')
TR_ = (TP('T'), TP('Rest', pack=True))
TUR = (TP('T'), TP('U'), TP('Rest', pack=True))
add('programs/W15/typelist.cpp', 'IndexOf<T, TypeList<T, Rest...>> vs IndexOf<T, TypeList<U, Rest...>>', None, None, None, 'first',
    (PS(TR_, (tT, cls('TypeList', [tT, expand(tp('Rest'))]))), PS(TUR, (tT, cls('TypeList', [tU, expand(tp('Rest'))]))), 'IndexOf'))
add('programs/W15/shapes.cpp', 'describe(const Shape auto&) vs describe(const Polygon auto&)',
    ft((TP('a1'),), lref(const(tp('a1'))), constraints=('Shape-req', 'a1')),
    ft((TP('a1'),), lref(const(tp('a1'))), constraints=('and', ('Shape-req', 'a1'), ('sides-req', 'a1'))), call(1), 'second')


# ---------------------------------------------------------------- the function-template grid

def call_deduce(form, arg):
    """Hand-written call deduction for the grid forms (score.py only).
    arg = (type, category). Returns (param type, qualification adjustment?) or None."""
    A, cat = arg
    if form == 'T':
        return (replace(A, cv=NOCV), False)
    if form == 'T pointer':
        return (A, False) if A.kind == 'ptr' else None
    if form == 'const T pointer':
        if A.kind != 'ptr':
            return None
        return (ptr(const(A.args[0])), 'const' not in A.args[0].cv)
    if form == 'T lref':
        return (lref(A), False) if cat == 'l' else None
    if form == 'const T lref':
        return (lref(const(A)), False)
    if form == 'T rref':
        return (lref(A), False) if cat == 'l' else (rref(A), False)
    if form == 'Box of T':
        return (A, False) if A.kind == 'cls' and A.name == 'Box' else None
    if form == 'pack':
        return (replace(A, cv=NOCV), False)


def seq_compare(p1, p2, cat):
    """ICS-26 (qualification adjustment as a longer sequence), ICS-28, ICS-30."""
    (t1, q1), (t2, q2) = p1, p2
    if q1 != q2:
        return 'second' if q1 else 'first'
    r1, r2 = t1.kind in ('lref', 'rref'), t2.kind in ('lref', 'rref')
    if r1 and r2:
        if cat != 'l' and t1.kind != t2.kind:
            return 'first' if t1.kind == 'rref' else 'second'
        a, b = t1.args[0], t2.args[0]
        if replace(a, cv=NOCV) == replace(b, cv=NOCV) and a.cv != b.cv:
            if a.cv < b.cv:
                return 'first'
            if b.cv < a.cv:
                return 'second'
    return 'tie'


FORMS = {
    'T': ft(T, tT), 'T pointer': ft(T, ptr(tT)), 'const T pointer': ft(T, ptr(const(tT))),
    'T lref': ft(T, lref(tT)), 'const T lref': ft(T, lref(const(tT))), 'T rref': ft(T, rref(tT)),
    'Box of T': ft(T, cls('Box', [tT])), 'pack': ft(Ts, expand(tp('Ts'))),
}
ARGS = OrderedDict([
    ('int lvalue', (fund('int'), 'l')), ('const int lvalue', (fund('int', C), 'l')),
    ('int prvalue', (fund('int'), 'pr')), ('int pointer', (ptr(fund('int')), 'l')),
    ('const int pointer', (ptr(fund('int', C)), 'l')), ('Box lvalue', (cls('Box', [fund('int')]), 'l')),
])


def read_grid(path):
    rows = {}
    cols = None
    for line in open(path):
        if not line.startswith('|'):
            if cols and rows:
                break
            continue
        cells = [c.strip() for c in line.strip().strip('|').split('|')]
        if cells[0] == '' and cols is None:
            cols = cells[1:]
        elif cells[0].startswith('---'):
            continue
        elif cols is not None:
            rows[cells[0]] = cells[1:]
    return cols, rows


def grid_function(path):
    lines = open(path).read().split('## Partial specializations')[0]
    tmp = '/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/I2/grid1.md'
    open(tmp, 'w').write(lines)
    cols, rows = read_grid(tmp)
    out = []
    for ci, col in enumerate(cols):
        a, b = col.split(' / ')
        for argname, arg in ARGS.items():
            pa, pb = call_deduce(a, arg), call_deduce(b, arg)
            if pa is None or pb is None:
                continue
            if seq_compare(pa, pb, arg[1]) != 'tie':
                continue
            first = rows[argname + ' → first'][ci] == 'accept'
            second = rows[argname + ' → second'][ci] == 'accept'
            want = 'first' if first else 'second' if second else 'neither'
            out.append(('%s, f(%s)' % (col, argname), FORMS[a], FORMS[b], call(1), want, None))
    return out


PSFORMS = {
    'T T': PS(T, (tT, tT)), 'T pointer U': PS(TU, (ptr(tT), tU)), 'T U pointer': PS(TU, (tT, ptr(tU))),
    'T pointer T pointer': PS(T, (ptr(tT), ptr(tT))), 'T int': PS(T, (tT, fund('int'))),
    'const T U': PS(TU, (const(tT), tU)),
}
PSARGS = OrderedDict([
    ('int int', (fund('int'), fund('int'))), ('int pointer int pointer', (ptr(fund('int')), ptr(fund('int')))),
    ('int pointer char', (ptr(fund('int')), fund('char'))), ('int int pointer', (fund('int'), ptr(fund('int')))),
    ('const int int', (fund('int', C), fund('int'))), ('char int', (fund('char'), fund('int'))),
])


def ps_matches(ps, args):
    env = Env(ps.tparams)
    try:
        match_list(list(ps.args), list(args), env, False)
        return True
    except Fail:
        return False


def grid_partial(path):
    text = open(path).read().split('## Partial specializations')[1]
    tmp = '/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/I2/grid2.md'
    open(tmp, 'w').write(text)
    cols, rows = read_grid(tmp)
    out = []
    for ci, col in enumerate(cols):
        a, b = col.split(' / ')
        for argname, args in PSARGS.items():
            if not (ps_matches(PSFORMS[a], args) and ps_matches(PSFORMS[b], args)):
                continue
            first = rows[argname + ' → first'][ci] == 'accept'
            second = rows[argname + ' → second'][ci] == 'accept'
            want = 'first' if first else 'second' if second else 'neither'
            out.append(('%s, Pair<%s>' % (col, argname), None, None, None, want, (PSFORMS[a], PSFORMS[b], 'Pair')))
    return out


G = '/home/ryanm/Code/tidal/tests/templates/ordering/grids/tables.md'
checks['templates/ordering/grids/tables.md (function templates)'] = grid_function(G)
checks['templates/ordering/grids/tables.md (partial specializations)'] = grid_partial(G)

# address-no-target-constraints-different-parameters.cpp: non-templates, ORD-13 conditions only
extra = []
f0 = FT('f', (), (), constraints=('Small', 'T'))
f1 = FT('f', (), (Param(fund('int')),), constraints=('and', ('Small', 'T'), ('size==1', 'T')))
tr = []
ok = ord13_conditions(f0, f1, Ctx('address'), tr)
extra.append(('overloading/overload-resolution/address-no-target-constraints-different-parameters.cpp',
              'f() requires Small vs f(int) requires Tiny: ORD-13 conditions hold?', ok, False))

total = matched = 0
print('| File | Checked | Matched | Mismatched |')
print('|---|---|---|---|')
detail = []
for file, cs in checks.items():
    m = 0
    for (label, f1, f2, ctx, want, partial) in cs:
        tr = []
        if partial:
            got = compare_partial(*partial, trace=tr)
        else:
            got = compare(f1, f2, ctx, tr)
        if got == want:
            m += 1
        else:
            detail.append('MISMATCH %s: %s: got %s, want %s\n    %s' % (file, label, got, want, '\n    '.join(tr)))
    total += len(cs)
    matched += m
    print('| %s | %d | %d | %d |' % (file, len(cs), m, len(cs) - m))
for file, label, got, want in extra:
    total += 1
    matched += got == want
    print('| %s | 1 | %d | %d |' % (file, got == want, got != want))
print('| total | %d | %d | %d |' % (total, matched, total - matched))
print()
print('\n'.join(detail))
if '-v' in sys.argv:
    for file, cs in checks.items():
        for c in cs:
            print(file, '|', c[0], '|', c[4])
