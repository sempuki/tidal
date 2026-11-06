"""Phase 2: score ranking.py (frozen in Phase 1) against Tidal's tests.

Each assertion the component can decide becomes a call of the component.
Part 1 scores the probe grids from their tables.md; part 2 (score_tests.py)
scores the individual test files.
"""
import re
import sys
from collections import Counter, defaultdict

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from ranking import *          # noqa

ROOT = '/home/ryanm/Code/tidal/tests/overloading'
I, L, S, D, F, LD, B, U, C = (fund('int'), fund('long'), fund('short'), fund('double'),
                              fund('float'), fund('long double'), fund('bool'),
                              fund('unsigned'), fund('char'))
const = lambda t: cq(t, 'const')
cv = lambda t: cq(t, 'const', 'volatile')


def parse_tables(path):
    """{section grid file: {row label: {column: cell}}}"""
    out = {}
    cur = None
    cols = None
    for line in open(path):
        m = re.search(r'`([\w-]+\.json)`', line)
        if line.startswith('Grid file') or line.startswith('## '):
            if m:
                cur = m.group(1)
                out[cur] = {}
                cols = None
            continue
        if cur and line.startswith('|'):
            cells = [c.strip() for c in line.strip().strip('|').split('|')]
            if cols is None:
                cols = cells[1:]
                continue
            if set(cells[0]) <= set('-'):
                continue
            out[cur][cells[0]] = dict(zip(cols, cells[1:]))
    return out


def norm(cell):
    if cell in ('accept', 'reject'):
        return cell
    return 'split'


def expect_pair(tab, row, col, first='first', second='second'):
    a = norm(tab[f'{row} → {first}'][col])
    b = norm(tab[f'{row} → {second}'][col])
    if 'split' in (a, b):
        return 'split'
    if a == 'accept' and b == 'reject':
        return 'first'
    if b == 'accept' and a == 'reject':
        return 'second'
    if a == b == 'reject':
        return 'neither'
    return 'both?'


def ambiguous_fault(res):
    """Caller check of ICS-8: the chosen function holds an ambiguous sequence."""
    for v in res.viable:
        if v.fn is res.fn:
            return any(s.kind == 'ambiguous' for s in v.seqs)
    return False


def outcome(res, f1, f2):
    if res.status == 'ok' and ambiguous_fault(res):
        return 'neither'
    if res.status == 'ok':
        return 'first' if res.fn is f1 else 'second' if res.fn is f2 else '?'
    return 'neither'


RESULTS = []        # (grid, row, column, expected, got, note)


def record(grid, row, col, exp, got, note=''):
    RESULTS.append((grid, row, col, exp, got, note))


def two(w, ta, tb, arg, la='a', lb='b'):
    f1 = Fn(f'f({la})', [] if ta == '...' else [ta], ellipsis=ta == '...')
    f2 = Fn(f'f({lb})', [] if tb == '...' else [tb], ellipsis=tb == '...')
    return outcome(resolve(w, [f1, f2], [arg]), f1, f2), f1, f2


def faults_bitfield(res_fn, ptype_of, row_is_bitfield):
    """Caller-side REF-8 check: a bit-field bound to a non-const lvalue reference."""
    return False


def run_pair_grid(name, tab, w, cols, rows, faults=None):
    labels = list(cols)
    for i in range(len(labels)):
        for j in range(i + 1, len(labels)):
            col = f'{labels[i]} / {labels[j]}'
            if col not in next(iter(tab.values())):
                continue
            for rl, arg in rows.items():
                exp = expect_pair(tab, rl, col)
                got, f1, f2 = two(w, cols[labels[i]], cols[labels[j]], arg, labels[i], labels[j])
                note = ''
                if faults and got != 'neither':
                    chosen = cols[labels[i]] if got == 'first' else cols[labels[j]]
                    if faults(rl, chosen):
                        note = f'component chose {got}; caller fault check rejects'
                        got = 'neither'
                record(name, rl, col, exp, got, note)


ICS_TAB = parse_tables(f'{ROOT}/conversion-sequences/grids/tables.md')
OVL_TAB = parse_tables(f'{ROOT}/overload-resolution/grids/tables.md')


def grid_arith():
    w = World()
    w.add_class('One'); w.add_class('Two')
    w.add_enum('Plain'); w.add_enum('Fixed', underlying='short'); w.add_enum('Scoped', scoped=True)
    cols = {'bool': B, 'short': S, 'int': I, 'long': L, 'float': F, 'double': D}
    rows = {'bool': lv(B), 'char': lv(C), 'short': lv(S), 'int': lv(I), 'unsigned': lv(U),
            'long': lv(L), 'float': lv(F), 'double': lv(D), 'long double': lv(LD),
            'enum': pr(enum('Plain')), 'enum : short': pr(enum('Fixed')),
            'enum class': pr(enum('Scoped')),
            # the component has no bit-field flag; the argument is a long long lvalue
            'long long : 3 bit-field': lv(fund('long long'))}
    run_pair_grid('arithmetic', ICS_TAB['arithmetic.json'], w, cols, rows)


def grid_pointers():
    w = World()
    w.add_class('Base'); w.add_class('Middle', bases=['Base']); w.add_class('Derived', bases=['Middle'])
    Bs, M, Dv = cls('Base'), cls('Middle'), cls('Derived')
    cols = {'void*': ptr(VOID), 'const void*': ptr(const(VOID)), 'Base*': ptr(Bs),
            'Middle*': ptr(M), 'const Middle*': ptr(const(M)), 'bool': B}
    rows = {'Derived*': lv(ptr(Dv)), 'const Derived*': lv(ptr(const(Dv))), 'Middle*': lv(ptr(M)),
            'Derived[2]': lv(arr(Dv, 2)), '0': Arg(I, 'prvalue', null=True),
            'nullptr': pr(NULLPTR_T)}
    run_pair_grid('pointers', ICS_TAB['pointers.json'], w, cols, rows)


def grid_mptr():
    w = World()
    w.add_class('Base'); w.add_class('Middle', bases=['Base']); w.add_class('Derived', bases=['Middle'])
    cols = {'int Base::*': mptr('Base', I), 'int Middle::*': mptr('Middle', I),
            'int Derived::*': mptr('Derived', I), 'bool': B}
    rows = {'&Base::x': pr(mptr('Base', I)), '&Middle::y': pr(mptr('Middle', I)),
            '0': Arg(I, 'prvalue', null=True), 'nullptr': pr(NULLPTR_T)}
    run_pair_grid('member-pointers', ICS_TAB['member-pointers.json'], w, cols, rows)


def grid_functions():
    w = World()
    fv = fnty(VOID)
    cols = {'pointer': ptr(fv), 'noexcept pointer': ptr(fnty(VOID, noexcept=True)),
            'lref': lref(fv), 'rref': rref(fv), 'bool': B}
    rows = {'run': lv(fv), 'stop (noexcept)': lv(fnty(VOID, noexcept=True))}
    run_pair_grid('functions', ICS_TAB['functions.json'], w, cols, rows)


def bitfield_fault(row, chosen):
    # REF-8, a caller check (ICS-8): a bit-field to a non-const lvalue reference
    return 'bit-field' in row and chosen.kind == 'ref' and chosen.rk == 'l' \
        and 'const' not in topcv(chosen.to)


def grid_references():
    w = World()
    cols = {'int': I, 'int lref': lref(I), 'const int lref': lref(const(I)),
            'const volatile int lref': lref(cv(I)), 'int rref': rref(I),
            'const int rref': rref(const(I)), 'const long lref': lref(const(L))}
    rows = {'int lvalue': lv(I), 'const int lvalue': lv(const(I)), 'int xvalue': xv(I),
            'int prvalue': pr(I), 'short lvalue': lv(S), 'int : 3 bit-field': lv(I)}
    run_pair_grid('references', ICS_TAB['references.json'], w, cols, rows, faults=bitfield_fault)


def grid_classes():
    w = World()
    w.add_class('Base'); w.add_class('Middle', bases=['Base']); w.add_class('Derived', bases=['Middle'])
    w.add_class('Box', ctors=[Fn('Box(int)', [I])])
    w.add_class('ToMiddle', convs=[Fn('operator Middle', [], cv=frozenset({'const'}), ret=cls('Middle'))])
    w.add_class('ToInt', convs=[Fn('operator int', [], cv=frozenset({'const'}), ret=I)])
    M = cls('Middle')
    cols = {'Base': cls('Base'), 'Middle': M, 'const Middle lref': lref(const(M)),
            'Middle rref': rref(M), 'long': L, 'Box': cls('Box'), 'ellipsis': '...'}
    rows = {'Derived lvalue': lv(cls('Derived')), 'Derived prvalue': pr(cls('Derived')),
            'Middle prvalue': pr(M), 'int': pr(I), 'ToMiddle': pr(cls('ToMiddle')),
            'ToInt': pr(cls('ToInt')), 'braced {d}': braced(lv(cls('Derived')))}
    run_pair_grid('classes', ICS_TAB['classes.json'], w, cols, rows)


def grid_user():
    w = World()
    w.add_class('Box', ctors=[Fn('Box(int)', [I])])
    w.add_class('Wide', ctors=[Fn('Wide(long)', [L])])
    k = frozenset({'const'})
    w.add_class('ToInt', convs=[Fn('operator int', [], cv=k, ret=I)])
    w.add_class('ToShort', convs=[Fn('operator short', [], cv=k, ret=S)])
    w.add_class('ToBoth', convs=[Fn('operator int', [], cv=k, ret=I), Fn('operator long', [], cv=k, ret=L)])
    cols = {'int': I, 'long': L, 'double': D, 'Box': cls('Box'), 'Wide': cls('Wide'), 'ellipsis': '...'}
    rows = {'int': pr(I), 'ToInt': pr(cls('ToInt')), 'ToShort': pr(cls('ToShort')),
            'ToBoth': pr(cls('ToBoth'))}
    run_pair_grid('user-defined', ICS_TAB['user-defined.json'], w, cols, rows)


def lists_world():
    w = World()
    w.add_class('Point', aggregate=True, fields=[('x', I), ('y', I)])
    w.add_class('Pair', ctors=[Fn('Pair(int,int)', [I, I])])
    return w


def grid_lists():
    w = lists_world()
    cols = {'int': I, 'initializer_list<int>': ilist(I), 'initializer_list<long>': ilist(L),
            'int[2] rref': rref(arr(I, 2)), 'int[] rref': rref(arr(I)), 'Point': cls('Point'),
            'Pair': cls('Pair')}
    one = lambda: pr(I)
    rows = {'empty {}': braced(), 'one {1}': braced(one()), 'two {1, 2}': braced(one(), one()),
            'three {1, 2, 3}': braced(one(), one(), one()), 'nested {{1}}': braced(braced(one())),
            'designated {.x = 1}': braced(one(), designators=['x'])}
    run_pair_grid('lists', ICS_TAB['lists.json'], w, cols, rows)


def grid_object():
    tab = ICS_TAB['object-parameter.json']
    k = frozenset({'const'})
    It = cls('Item')
    cols = {'no qualifier / Item lref': (E, '', lref(It)),
            'no qualifier / Item rref': (E, '', rref(It)),
            'const / const Item lref': (k, '', lref(const(It))),
            'const / const Item rref': (k, '', rref(const(It))),
            'const lref / const Item rref': (k, '&', rref(const(It))),
            'lref / Item rref': (E, '&', rref(It)),
            'rref / Item lref': (E, '&&', lref(It)),
            'rref / const Item lref': (E, '&&', lref(const(It)))}
    rows = {'Item lvalue': lv(It), 'const Item lvalue': lv(const(It)), 'Item prvalue': pr(It)}
    for col, (q, rq, pt) in cols.items():
        w = World()
        w.add_class('Item')
        m = Fn('member ~', [], kind='member', owner='Item', cv=q, refq=rq)
        n = Fn('non-member ~', [pt])
        for rl, a in rows.items():
            exp = expect_pair(tab, rl, col, 'member', 'non-member')
            got = outcome(resolve(w, [m, n], [a], has_object=True), m, n)
            record('object-parameter', rl, col, exp, got)


def grid_order():
    tab = ICS_TAB['order.json']
    w = World()
    w.add_class('Base'); w.add_class('Middle', bases=['Base']); w.add_class('Derived', bases=['Middle'])
    w.add_enum('Level', underlying='short')
    Bs, M, Dv = cls('Base'), cls('Middle'), cls('Derived')
    cols = {'const short lref / int rref': (lref(const(S)), rref(I)),
            'void* const lref / bool rref': (lref(const(ptr(VOID))), rref(B)),
            'const Middle lref / Base rref': (lref(const(M)), rref(Bs)),
            'Base* const lref / void* rref': (lref(const(ptr(Bs))), rref(ptr(VOID))),
            'Middle* const lref / Base* rref': (lref(const(ptr(M))), rref(ptr(Bs))),
            'int Middle::* const lref / int Derived::* rref': (lref(const(mptr('Middle', I))), rref(mptr('Derived', I))),
            'const double lref / long double rref': (lref(const(D)), rref(LD)),
            'const int* const lref / const volatile int* rref': (lref(const(ptr(const(I)))), rref(ptr(cv(I))))}
    rows = {'low (enum : short)': pr(enum('Level')), '&n (int*)': pr(ptr(I)),
            'Derived{}': pr(Dv), '&d (Derived*)': pr(ptr(Dv)), '&Base::x': pr(mptr('Base', I))}
    # 1.0f64 needs an extended floating-point type; the component has none.
    for col, (pa, pb) in cols.items():
        for rl, a in rows.items():
            exp = expect_pair(tab, rl, col)
            got, _, _ = two(w, pa, pb, a)
            record('order', rl, col, exp, got)


def grid_list_refs():
    w = World()
    cols = {'int lref': lref(I), 'const int lref': lref(const(I)), 'int rref': rref(I),
            'const int rref': rref(const(I)), 'const long lref': lref(const(L)), 'long': L}
    rows = {'{i} int lvalue': braced(lv(I)), '{ci} const int lvalue': braced(lv(const(I))),
            '{1} prvalue': braced(pr(I)), '{s} short lvalue': braced(lv(S))}
    run_pair_grid('list-references', ICS_TAB['list-references.json'], w, cols, rows)


# ---------------------------------------------------------------- overload-resolution grids

def grid_viability():
    tab = OVL_TAB['viability.json']
    w = World()
    cols = {'none': lambda n: Fn('f()', []), 'one': lambda n: Fn('f(int)', [I]),
            'one default': lambda n: Fn('f(int=0)', [I], defaults=1),
            'two': lambda n: Fn('f(int,int)', [I, I]),
            'one then default': lambda n: Fn('f(int,int=0)', [I, I], defaults=1),
            'ellipsis': lambda n: Fn('f(...)', [], ellipsis=True),
            'one then ellipsis': lambda n: Fn('f(int,...)', [I], ellipsis=True),
            # deduction gives one int per argument (DED-20)
            'pack': lambda n: Fn('f(T...)', [I] * n, template=True),
            'one then pack': lambda n: (Fn('f(int,T...)', [I] + [I] * (n - 1), template=True)
                                        if n >= 1 else None)}
    rows = {'no argument': 0, 'one argument': 1, 'two arguments': 2, 'three arguments': 3}
    for col, mk in cols.items():
        for rl, n in rows.items():
            exp = norm(tab[rl][col])
            f = mk(n)
            got = 'reject' if f is None else (
                'accept' if resolve(w, [f], [pr(I)] * n).status == 'ok' else 'reject')
            record('viability', rl, col, exp, got)


def grid_object_call():
    tab = OVL_TAB['object-call.json']
    k = frozenset({'const'})
    cols = {'lref call': dict(params=[I], refq='&'), 'const lref call': dict(params=[I], refq='&', cv=k),
            'rref call': dict(params=[I], refq='&&'), 'long call': dict(params=[L], refq='&', cv=k)}
    rows = {'S lvalue': lv(cls('S')), 'const S lvalue': lv(const(cls('S'))), 'S prvalue': pr(cls('S'))}
    labels = list(cols)
    for i in range(len(labels)):
        for j in range(i + 1, len(labels)):
            col = f'{labels[i]} / {labels[j]}'
            w = World()
            w.add_class('S')
            f1 = Fn('first', kind='member', owner='S', **cols[labels[i]])
            f2 = Fn('second', kind='member', owner='S', **cols[labels[j]])
            for rl, a in rows.items():
                exp = expect_pair(tab, rl, col)
                got = outcome(resolve(w, [f1, f2], [a, pr(I)], has_object=True), f1, f2)
                record('object-call', rl, col, exp, got)


PROMOTED_ARITH = ['int', 'unsigned', 'long', 'unsigned long', 'long long', 'unsigned long long',
                  'float', 'double', 'long double']


def builtin_pairs(op):
    return [Fn(f'builtin {op}({a},{b})', [fund(a), fund(b)], kind='builtin')
            for a in PROMOTED_ARITH for b in PROMOTED_ARITH]


def grid_operators():
    tab = OVL_TAB['operators.json']
    k = frozenset({'const'})
    X = cls('X')
    kinds = ['member', 'non-member', 'built-in', 'rewritten', 'swapped']

    def cands(kind):
        if kind == 'member':
            return [Fn('member', [I], kind='member', owner='X', cv=k)]
        if kind == 'non-member':
            return [Fn('non-member', [lref(const(X)), I])]
        if kind == 'built-in':
            return builtin_pairs('<')
        if kind == 'rewritten':
            return [Fn('rewritten', [I], kind='member', owner='X', cv=k, rewritten=True)]
        if kind == 'swapped':
            # operator<=>(int, const X&) with the operands swapped
            return [Fn('swapped', [lref(const(X)), I], rewritten=True, swapped=True)]

    for i in range(len(kinds)):
        for j in range(i + 1, len(kinds)):
            col = f'{kinds[i]} / {kinds[j]}'
            if col not in tab['x < 1 → member']:
                continue
            w = World()
            convs = []
            if 'built-in' in (kinds[i], kinds[j]):
                convs = [Fn('operator int', [], cv=k, ret=I)]
            w.add_class('X', convs=convs)
            cs = cands(kinds[i]) + cands(kinds[j])
            res = resolve(w, cs, [lv(X), pr(I)], has_object=True)
            accepted = [r for r in kinds if norm(tab[f'x < 1 → {r}'][col]) == 'accept']
            exp = accepted[0] if len(accepted) == 1 else ('neither' if not accepted else 'split')
            if res.status != 'ok':
                got = 'neither'
            else:
                n = res.fn.name
                got = 'built-in' if n.startswith('builtin') else n
            record('operators', 'x < 1', col, exp, got)


def grid_templates():
    tab = OVL_TAB['templates.json']
    w = World()
    n_l, one, oneL, p_l = lv(I), pr(I), pr(L), lv(ptr(I))
    PI = ptr(I)
    # Deduction (DED) is outside the component; these are the specializations.
    # None = deduction fails or the constraint is not satisfied.
    spec = {
        'plain int': {'all': I},
        'plain const int lref': {'all': lref(const(I))},
        'plain long': {'all': L},
        'template value': {'int lvalue': I, 'int prvalue': I, 'long': L, 'int pointer': PI},
        'template const lref': {'int lvalue': lref(const(I)), 'int prvalue': lref(const(I)),
                                'long': lref(const(L)), 'int pointer': lref(const(PI))},
        'template pointer': {'int pointer': PI},
        'template forwarding': {'int lvalue': lref(I), 'int prvalue': rref(I), 'long': rref(L),
                                'int pointer': lref(PI)},
        'constrained template': {'int lvalue': I, 'int prvalue': I, 'long': L, 'int pointer': PI},
        'ellipsis': {'all': '...'},
    }
    shape = {'template value': 'T', 'template const lref': 'const T&', 'template pointer': 'T*',
             'template forwarding': 'T&&', 'constrained template': 'T'}

    def order(f1, f2):
        # ORD is a black box answered by the caller (outside knowledge).
        a, b = shape[f1.label], shape[f2.label]
        if a == b:
            return 'equal'
        if a == 'T*':
            return 'first'
        if b == 'T*':
            return 'second'
        if {a, b} == {'const T&', 'T&&'}:
            return 'first' if a == 'const T&' else 'second'
        return None

    def more_constrained(f1, f2):
        return f1.label == 'constrained template' and f2.label == 'template value'

    rows = {'int lvalue': n_l, 'int prvalue': one, 'long': oneL, 'int pointer': p_l}
    labels = list(spec)
    for i in range(len(labels)):
        for j in range(i + 1, len(labels)):
            col = f'{labels[i]} / {labels[j]}'
            if col not in tab['int lvalue → first']:
                continue
            for rl, a in rows.items():
                fs = []
                for lab, nm in ((labels[i], 'first'), (labels[j], 'second')):
                    t = spec[lab].get('all', spec[lab].get(rl))
                    if t is None:
                        continue
                    f = Fn(nm, [] if t == '...' else [t], ellipsis=t == '...',
                           template=lab in shape)
                    f.label = lab
                    fs.append(f)
                res = resolve(w, fs, [a], order=order, more_constrained=more_constrained)
                got = res.fn.name if res.status == 'ok' else 'neither'
                record('templates', rl, col, expect_pair(tab, rl, col), got)


def grid_reference_kind():
    tab = OVL_TAB['reference-kind.json']
    Fty = fnty(I)
    cols = {'plain lref': (lref(Fty), False), 'plain rref': (rref(Fty), False),
            'template lref': (lref(Fty), True), 'template rref': (rref(Fty), True)}
    labels = list(cols)
    for i in range(len(labels)):
        for j in range(i + 1, len(labels)):
            col = f'{labels[i]} / {labels[j]}'
            for rl, target in (('lref target', lref(Fty)), ('rref target', rref(Fty))):
                w = World()
                f1 = Fn('first', ret=cols[labels[i]][0], template=cols[labels[i]][1], cv=frozenset({'const'}))
                f2 = Fn('second', ret=cols[labels[j]][0], template=cols[labels[j]][1], cv=frozenset({'const'}))
                w.add_class('Source', convs=[f1, f2])
                s = implicit_seq(w, pr(cls('Source')), target)
                if s is None or s.kind != 'user':
                    got = 'neither'
                else:
                    got = s.func.name
                record('reference-kind', rl, col, expect_pair(tab, rl, col), got)


def grid_initialization():
    tab = OVL_TAB['initialization.json']
    k = frozenset({'const'})
    Src, T = cls('Src'), cls('T')

    def world(col):
        w = World()
        if col == 'converting constructor':
            w.add_class('Src'); w.add_class('T', ctors=[Fn('T(const Src&)', [lref(const(Src))])])
        elif col == 'explicit constructor':
            w.add_class('Src'); w.add_class('T', ctors=[Fn('T(const Src&)', [lref(const(Src))], explicit=True)])
        elif col == 'conversion function':
            w.add_class('T', ctors=[Fn('T()', [])]); w.add_class('Src', convs=[Fn('operator T', [], cv=k, ret=T)])
        elif col == 'explicit conversion function':
            w.add_class('T', ctors=[Fn('T()', [])]); w.add_class('Src', convs=[Fn('operator T', [], cv=k, ret=T, explicit=True)])
        elif col == 'conversion to derived':
            w.add_class('T', ctors=[Fn('T()', [])]); w.add_class('D', bases=['T'])
            w.add_class('Src', convs=[Fn('operator D', [], cv=k, ret=cls('D'))])
        elif col == 'constructor and conversion function':
            w.add_class('T', ctors=[Fn('T(const Src&)', [lref(const(Src))])])
            w.add_class('Src', convs=[Fn('operator T', [], cv=k, ret=T)])
        elif col == 'explicit constructor and conversion function':
            w.add_class('T', ctors=[Fn('T(const Src&)', [lref(const(Src))], explicit=True)])
            w.add_class('Src', convs=[Fn('operator T', [], cv=k, ret=T)])
        return w

    for col in tab['copy init']:
        w = world(col)
        # copy initialization and the argument both form the sequence of OVL-14 / ICS-4.
        for rl in ('copy init', 'argument', 'return'):
            s = implicit_seq(w, lv(Src), T)
            got = 'accept' if s is not None and s.kind in ('standard', 'user') else 'reject'
            record('initialization', rl, col, norm(tab[rl][col]), got)


def grid_three_way():
    tab = OVL_TAB['three-way.json']
    k = frozenset({'const'})
    to = {'To bool': B, 'To char': C, 'To short': S, 'To int': I, 'To unsigned': U,
          'To long': L, 'To double': D}
    for rl, ta in to.items():
        for col, tb in to.items():
            exp = norm(tab[rl][col])
            w = World()
            w.add_class('A', convs=[Fn('operator a', [], cv=k, ret=ta)])
            w.add_class('Bc', convs=[Fn('operator b', [], cv=k, ret=tb)])
            res = resolve(w, builtin_pairs('<=>'), [pr(cls('A')), pr(cls('Bc'))])
            got = 'accept' if res.status == 'ok' else 'reject'
            note = ''
            if exp == 'reject' and got == 'accept':
                note = f'chose {res.fn}; OVL-11 check of the operator rules is the caller\'s'
                got = 'caller'
            record('three-way', rl, col, exp, got, note)


def grid_compound():
    tab = OVL_TAB['compound-integral.json']
    k = frozenset({'const'})
    ops = {'plus assign': 'A', 'remainder assign': 'I', 'and assign': 'I', 'or assign': 'I',
           'shift assign': 'I'}
    lefts = ['bool', 'char', 'signed char', 'unsigned char', 'wchar_t', 'char8_t', 'char16_t',
             'short', 'int', 'unsigned long']
    INTEGRAL_LEFT = INTEGRAL
    PROM_INT = ['int', 'unsigned', 'long', 'unsigned long', 'long long', 'unsigned long long']
    for rl in lefts:
        for col, kind in ops.items():
            w = World()
            w.add_class('ToInt', convs=[Fn('operator int', [], cv=k, ret=I)])
            rights = PROMOTED_ARITH if kind == 'A' else PROM_INT
            lhs_types = (INTEGRAL + FLOATING) if kind == 'A' else INTEGRAL
            cands = [Fn(f'builtin({x}&,{y})', [lref(fund(x)), fund(y)], kind='builtin')
                     for x in lhs_types for y in rights]
            res = resolve(w, cands, [lv(fund(rl)), pr(cls('ToInt'))])
            got = 'accept' if res.status == 'ok' else 'reject'
            record('compound-integral', rl, col, norm(tab[rl][col]), got)


GRIDS = [grid_arith, grid_pointers, grid_mptr, grid_functions, grid_references, grid_classes,
         grid_user, grid_lists, grid_object, grid_order, grid_list_refs, grid_viability,
         grid_object_call, grid_operators, grid_templates, grid_reference_kind,
         grid_initialization, grid_three_way, grid_compound]


def main():
    for g in GRIDS:
        g()
    by = defaultdict(Counter)
    lines = []
    for grid, row, col, exp, got, note in RESULTS:
        if exp == 'split':
            k = 'split'
        elif got == 'caller':
            k = 'caller'
        else:
            k = 'pass' if exp == got else 'FAIL'
        by[grid][k] += 1
        if k == 'FAIL':
            lines.append(f'FAIL {grid} | {row} | {col} | expected {exp} | got {got} {note}')
    out = ['# Grid scoring of ranking.py (sha1 in ranking.sha1)', '']
    tot = Counter()
    for grid, c in by.items():
        out.append(f'{grid:20s} pass {c["pass"]:4d}  fail {c["FAIL"]:4d}  split {c["split"]:4d}  caller {c["caller"]:3d}')
        tot.update(c)
    out.append(f'{"TOTAL":20s} pass {tot["pass"]:4d}  fail {tot["FAIL"]:4d}  split {tot["split"]:4d}  caller {tot["caller"]:3d}')
    out.append('')
    out.extend(lines)
    return '\n'.join(out)


if __name__ == '__main__':
    print(main())
