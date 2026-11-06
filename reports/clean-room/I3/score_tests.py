"""Phase 2, part 2: individual test files turned into calls of ranking.py.

Each case names the test file, the assertion, the expected result, and the
component's result.  For a test whose expectation is "reject" because of a
fault that ICS-8 leaves to the chosen function, the case checks the choice
that the test's note gives, since the component decides only the choice.
"""
import sys
sys.path.insert(0, __file__.rsplit('/', 1)[0])
from ranking import *          # noqa
from score import I, L, S, D, F, LD, B, U, C, const, cv, ambiguous_fault  # noqa

CASES = []      # (file, assertion, expected, got, note)
NOTREP = []     # (file, reason): assertions the component's model cannot represent
k = frozenset({'const'})


def fn(name, *params, **kw):
    ell = kw.pop('ellipsis', False)
    return Fn(name, list(params), ellipsis=ell, **kw)


def call(w, cands, args, **kw):
    res = resolve(w, cands, args, **kw)
    if res.status == 'ok':
        return res.fn.name
    return res.status


def case(file, what, expected, got, note=''):
    CASES.append((file, what, expected, got, note))


def notrep(file, why):
    NOTREP.append((file, why))


def conv_init(w, arg, target):
    """Initialization of target from arg by a conversion function (ICS-37)."""
    s = implicit_seq(w, arg, target)
    if s is None:
        return 'none'
    if s.kind == 'user':
        return s.func.name
    return s.kind


def chain(w, names=('Base', 'Middle', 'Derived')):
    w.add_class(names[0])
    w.add_class(names[1], bases=[names[0]])
    w.add_class(names[2], bases=[names[1]])


# ---------------------------------------------------------------- conversion-sequences, batch 1

def batch1():
    f = 'ambiguous-loses-to-standard.cpp'
    w = World()
    w.add_class('Left', ctors=[fn('Left(int)', I)]); w.add_class('Right', ctors=[fn('Right(int)', I)])
    w.add_class('Both', ctors=[fn('Both(Left)', cls('Left')), fn('Both(Right)', cls('Right'))])
    case(f, 'take(1)', 'Two', call(w, [fn('One', cls('Both')), fn('Two', I)], [pr(I)]))
    w.add_class('Source', convs=[fn('op int', ret=I), fn('op long', ret=L)])
    case(f, 'look(Source{})', 'Two', call(w, [fn('One', D), fn('Two', cls('Source'))], [pr(cls('Source'))]))

    f = 'ambiguous-ties-user-defined.cpp'
    w = World()
    w.add_class('Source', convs=[fn('op int', ret=I), fn('op long', ret=L)])
    w.add_class('Wrap', ctors=[fn('Wrap(Source)', cls('Source'))])
    case(f, 'take(Source{}, 0)', 'Two',
         call(w, [fn('One', D, L), fn('Two', cls('Wrap'), I)], [pr(cls('Source')), pr(I)]))

    w = World()
    f = 'array-fewer-elements-const-lvalue.cpp'
    case(f, 'take({1.0})', 'One', call(w, [fn('One', lref(const(arr(D, 1)))), fn('Two', lref(const(arr(D, 5))))], [braced(pr(D))]))
    f = 'array-fewer-elements.cpp'
    case(f, 'take({1,2})', 'One', call(w, [fn('One', rref(arr(I, 2))), fn('Two', rref(arr(I, 3)))], [braced(pr(I), pr(I))]))
    f = 'array-known-bound-const-lvalue.cpp'
    case(f, 'take({1L,2L,3L})', 'One', call(w, [fn('One', lref(const(arr(L, 3)))), fn('Two', lref(const(arr(L))))], [braced(pr(L), pr(L), pr(L))]))
    f = 'array-known-bound.cpp'
    case(f, 'take({1,2})', 'One', call(w, [fn('One', rref(arr(I, 2))), fn('Two', rref(arr(I)))], [braced(pr(I), pr(I))]))
    f = 'array-known-bound-direct.cpp'
    case(f, 'take(row)', 'One', call(w, [fn('One', lref(arr(I, 2))), fn('Two', lref(arr(I)))], [lv(arr(I, 2))]))
    f = 'array-known-bound-direct-cv.cpp'
    case(f, 'take(row)', 'One', call(w, [fn('One', lref(arr(I, 2))), fn('Two', lref(const(arr(I))))], [lv(arr(I, 2))]))

    f = 'array-pointer-tie-breaker.cpp'
    w = World(); chain(w)
    case(f, 'take(items)', 'Two', call(w, [fn('One', ptr(VOID)), fn('Two', ptr(cls('Base')))], [lv(arr(cls('Derived'), 2))]))
    case(f, 'near(items)', 'Two', call(w, [fn('One', ptr(cls('Base'))), fn('Two', ptr(cls('Middle')))], [lv(arr(cls('Derived'), 2))]))

    def assign_world(extra_member):
        w = World()
        convs = [fn('op int&', ret=lref(I))]
        w.add_class('Cell', convs=convs)
        Cl = cls('Cell')
        ms = [fn('Cell=(const Cell&)', lref(const(Cl)), kind='member', owner='Cell'),
              fn('Cell=(Cell&&)', rref(Cl), kind='member', owner='Cell')]
        if extra_member:
            ms.append(fn('Cell=(long)', L, kind='member', owner='Cell'))
        bi = [fn(f'builtin=({x}&,{y})', lref(fund(x)), fund(y), kind='builtin', std_only=(0,))
              for x in ('int', 'long') for y in ('int', 'long', 'double')]
        return w, ms + bi
    f = 'assignment-left-no-builtin.cpp'
    w, cs = assign_world(False)
    case(f, 'c = 1', 'no viable function', call(w, cs, [lv(cls('Cell')), pr(I)], has_object=True))
    f = 'assignment-left-standard-only.cpp'
    w, cs = assign_world(True)
    case(f, 'c = 5', 'Cell=(long)', call(w, cs, [lv(cls('Cell')), pr(I)], has_object=True))

    f = 'base-pointer-over-void.cpp'
    w = World(); w.add_class('Base'); w.add_class('Derived', bases=['Base'])
    case(f, 'take(d)', 'One', call(w, [fn('One', ptr(cls('Base'))), fn('Two', ptr(VOID))], [lv(ptr(cls('Derived')))]))

    f = 'bit-field-promotion.cpp'
    case(f, 'take(bits.v)', 'One', call(World(), [fn('One', I), fn('Two', L)], [lv(fund('long long'))]),
         'the component has no bit-field argument; long long stands in')

    f = 'chosen-ambiguous-base.cpp'
    w = World(); w.add_class('Base'); w.add_class('Left', bases=['Base']); w.add_class('Right', bases=['Base'])
    w.add_class('Joined', bases=['Left', 'Right'])
    case(f, 'take(j) chooses take(Base*)', 'One', call(w, [fn('One', ptr(cls('Base'))), fn('Two', ellipsis=True)], [lv(ptr(cls('Joined')))]))

    f = 'chosen-ambiguous-constructor-conversion.cpp'
    w = World()
    w.add_class('Target', ctors=[fn('Target()'), fn('Target(Source&)', lref(cls('Source')))])
    w.add_class('Source', convs=[fn('op Target', ret=cls('Target'))])
    r = resolve(w, [fn('One', cls('Target')), fn('Two', ellipsis=True)], [lv(cls('Source'))])
    case(f, 'take(s) chooses take(Target) with an ambiguous sequence', 'One+ambiguous',
         (r.fn.name if r.fn else r.status) + ('+ambiguous' if ambiguous_fault(r) else ''))

    f = 'chosen-ambiguous.cpp'
    w = World(); w.add_class('Source', convs=[fn('op int', ret=I), fn('op long', ret=L)])
    r = resolve(w, [fn('One', D), fn('Two', ellipsis=True)], [pr(cls('Source'))])
    case(f, 'take(Source{})', 'One+ambiguous', (r.fn.name if r.fn else r.status) + ('+ambiguous' if ambiguous_fault(r) else ''))

    f = 'chosen-ambiguous-tie.cpp'
    w.add_class('Wrap', ctors=[fn('Wrap(Source)', cls('Source'))])
    r = resolve(w, [fn('One', D, I), fn('Two', cls('Wrap'), L)], [pr(cls('Source')), pr(I)])
    case(f, 'take(Source{}, 0)', 'One+ambiguous', (r.fn.name if r.fn else r.status) + ('+ambiguous' if ambiguous_fault(r) else ''))

    f = 'chosen-bit-field.cpp'
    case(f, 'take(flags.mode) chooses take(int&)', 'One', call(World(), [fn('One', lref(I)), fn('Two', L)], [lv(I)]))

    f = 'chosen-deleted-constructor.cpp'
    w = World(); w.add_class('Gate', ctors=[fn('Gate(int)', I, deleted=True)])
    case(f, 'take(1) chooses take(Gate)', 'One', call(w, [fn('One', cls('Gate')), fn('Two', ellipsis=True)], [pr(I)]))
    f = 'chosen-deleted-conversion-function.cpp'
    w = World(); w.add_class('Source', convs=[fn('op int', ret=I, deleted=True)])
    case(f, 'take(Source{}) chooses take(int)', 'One', call(w, [fn('One', I), fn('Two', ellipsis=True)], [pr(cls('Source'))]))

    f = 'chosen-designator-order.cpp'
    w = World()
    w.add_class('Point', aggregate=True, fields=[('x', I), ('y', I)])
    w.add_class('Swapped', aggregate=True, fields=[('y', I), ('x', I)])
    case(f, 'take({.x=1,.y=2}, 0) chooses take(Swapped,int)', 'Two',
         call(w, [fn('One', cls('Point'), L), fn('Two', cls('Swapped'), I)],
              [braced(pr(I), pr(I), designators=['x', 'y']), pr(I)]))

    f = 'chosen-explicit-copy-list.cpp'
    w = World()
    w.add_class('Strict', ctors=[fn('Strict(int,int)', I, I, explicit=True)])
    w.add_class('Loose', ctors=[fn('Loose(long,long)', L, L)])
    case(f, 'take({1,2}, 0) chooses take(Strict,int)', 'One',
         call(w, [fn('One', cls('Strict'), I), fn('Two', cls('Loose'), L)], [braced(pr(I), pr(I)), pr(I)]))

    f = 'chosen-list-ambiguous.cpp'
    w = World(); w.add_class('Split', ctors=[fn('Split(int,long)', I, L), fn('Split(long,int)', L, I)])
    r = resolve(w, [fn('One', cls('Split')), fn('Two', ellipsis=True)], [braced(pr(I), pr(I))])
    case(f, 'take({1,1})', 'One+ambiguous', (r.fn.name if r.fn else r.status) + ('+ambiguous' if ambiguous_fault(r) else ''))

    f = 'chosen-narrowing.cpp'
    w = World(); w.add_class('Real', ctors=[fn('Real(double)', D)])
    case(f, 'take({d}) chooses take(int)', 'One', call(w, [fn('One', I), fn('Two', cls('Real'))], [braced(lv(D))]))

    f = 'chosen-private-base.cpp'
    w = World(); w.add_class('Base'); w.add_class('Derived', bases=['Base'])
    case(f, 'take(d) chooses take(Base*)', 'One', call(w, [fn('One', ptr(cls('Base'))), fn('Two', ellipsis=True)], [lv(ptr(cls('Derived')))]))

    f = 'conversion-choice-class-value.cpp'
    w = World(); chain(w)
    w.add_class('Source', convs=[fn('op Middle', ret=cls('Middle'), cv=k), fn('op Derived', ret=cls('Derived'), cv=k)])
    case(f, 'Base b = Source{}', 'op Middle', conv_init(w, pr(cls('Source')), cls('Base')))

    notrep('conversion-choice-equal-rank-extended-target.cpp', 'needs an extended floating-point type (ICS-44)')
    notrep('conversion-choice-equal-rank-source.cpp', 'needs extended floating-point types (ICS-44)')

    f = 'conversion-choice-nearest-source.cpp'
    w = World(); chain(w)
    w.add_class('Ptr', convs=[fn('op Derived*', ret=ptr(cls('Derived')), cv=k), fn('op Middle*', ret=ptr(cls('Middle')), cv=k)])
    w.add_class('Ref', convs=[fn('op Derived&', ret=lref(cls('Derived')), cv=k), fn('op Middle&', ret=lref(cls('Middle')), cv=k)])
    w.add_class('Mem', convs=[fn('op int Base::*', ret=mptr('Base', I), cv=k), fn('op int Middle::*', ret=mptr('Middle', I), cv=k)])
    case(f, 'static_cast<Base*>(Ptr{})', 'op Middle*', conv_init(w, pr(cls('Ptr')), ptr(cls('Base'))))
    case(f, 'Base& r = Ref{}', 'op Middle&', conv_init(w, pr(cls('Ref')), lref(cls('Base'))))
    case(f, 'int Derived::* m = Mem{}', 'op int Middle::*', conv_init(w, pr(cls('Mem')), mptr('Derived', I)))

    f = 'conversion-choice-promotion-result.cpp'
    w = World(); w.add_class('Source', convs=[fn('op float', ret=F, cv=k), fn('op int', ret=I, cv=k)])
    case(f, 'double d = Source{}', 'op float', conv_init(w, pr(cls('Source')), D))

    f = 'conversion-choice-second-part.cpp'
    w = World(); w.add_class('Source', convs=[fn('op int', ret=I), fn('op long', ret=L)])
    w.add_class('Small', convs=[fn('op short', ret=S, cv=k), fn('op long', ret=L, cv=k)])
    case(f, 'long l = Source{}', 'op long', conv_init(w, pr(cls('Source')), L))
    case(f, 'int chosen = Small{}', 'op short', conv_init(w, pr(cls('Small')), I))

    f = 'conversion-choice-void-const.cpp'
    w = World(); chain(w, ('Top', 'Middle', 'Bottom'))
    w.add_class('Source', convs=[fn('op Bottom*', ret=ptr(cls('Bottom')), cv=k), fn('op Middle*', ret=ptr(cls('Middle')), cv=k)])
    case(f, 'const void* p = Source{}', 'op Middle*', conv_init(w, pr(cls('Source')), ptr(const(VOID))))
    f = 'conversion-choice-void.cpp'
    w = World(); w.add_class('Base'); w.add_class('Derived', bases=['Base'])
    w.add_class('Source', convs=[fn('op Derived*', ret=ptr(cls('Derived')), cv=k), fn('op Base*', ret=ptr(cls('Base')), cv=k)])
    case(f, 'void* p = Source{}', 'op Base*', conv_init(w, pr(cls('Source')), ptr(VOID)))

    f = 'double-brace-standard-only.cpp'
    w = World(); w.add_class('Pair', ctors=[fn('Pair(int,int)', I, I)])
    w.add_class('Point', aggregate=True, fields=[('x', I), ('y', I)])
    w.add_class('Holder', aggregate=True, fields=[('p', cls('Point'))])
    case(f, 'take({{1,2}})', 'Two', call(w, [fn('One', cls('Pair')), fn('Two', cls('Holder'))], [braced(braced(pr(I), pr(I)))]))

    f = 'ellipsis-braced-list.cpp'
    w = World(); w.add_class('Label', ctors=[fn('Label(int)', I)]); w.add_class('Item')
    case(f, 'show({item})', 'no viable function', call(w, [fn('int', cls('Label')), fn('long', ellipsis=True)], [braced(lv(cls('Item')))]))
    f = 'ellipsis-braced-list-other-wins.cpp'
    case(f, 'show({1})', 'int', call(World(), [fn('long', ellipsis=True), fn('int', I, ellipsis=True)], [braced(pr(I))]))
    f = 'ellipsis-sequence.cpp'
    w = World(); w.add_class('Tag')
    case(f, 'any(1, 2.0, Tag{})', 'One', call(w, [fn('One', ellipsis=True)], [pr(I), pr(D), pr(cls('Tag'))]))

    f = 'fixed-enumeration-before-rvalue-reference.cpp'
    w = World(); w.add_enum('Level', underlying='short')
    case(f, 'take(low)', 'One', call(w, [fn('One', lref(const(S))), fn('Two', rref(I))], [pr(enum('Level'))]))
    f = 'fixed-enumeration.cpp'
    w = World(); w.add_enum('Level', underlying='short'); w.add_enum('Code', underlying='unsigned char')
    case(f, 'take(low)', 'One', call(w, [fn('One', S), fn('Two', I)], [pr(enum('Level'))]))
    case(f, 'code(none)', 'One', call(w, [fn('One', fund('unsigned char')), fn('Two', I)], [pr(enum('Code'))]))
    f = 'fixed-enumeration-promotion.cpp'
    w = World(); w.add_enum('Level', underlying='short'); w.add_enum('Code', underlying='int')
    case(f, 'take(low)', 'One', call(w, [fn('One', S), fn('Two', L)], [pr(enum('Level'))]))
    case(f, 'wide(low)', 'One', call(w, [fn('One', I), fn('Two', L)], [pr(enum('Level'))]))
    case(f, 'code(none)', 'One', call(w, [fn('One', I), fn('Two', L)], [pr(enum('Code'))]))
    f = 'fixed-enumeration-promotion-unsigned-char.cpp'
    w = World(); w.add_enum('Flag', underlying='unsigned char')
    case(f, 'narrow(off)', 'One', call(w, [fn('One', fund('unsigned char')), fn('Two', U)], [pr(enum('Flag'))]))
    case(f, 'wide(off)', 'One', call(w, [fn('One', I), fn('Two', D)], [pr(enum('Flag'))]))

    notrep('floating-before-rvalue-reference.cpp', 'needs std::float64_t (ICS-35)')
    notrep('floating-equal-rank.cpp', 'needs std::float64_t (ICS-35)')

    f = 'function-lvalue-reference-cast.cpp'
    ft = fnty(I, [I])
    case(f, 'take(static_cast<int(&&)(int)>(twice))', 'One', call(World(), [fn('One', lref(ft)), fn('Two', rref(ft))], [lv(ft)]))
    f = 'function-lvalue-reference.cpp'
    case(f, 'take(run)', 'One', call(World(), [fn('One', lref(fnty(VOID))), fn('Two', rref(fnty(VOID)))], [lv(fnty(VOID))]))


BATCHES = [batch1]


def main():
    for b in BATCHES:
        b()
    out = []
    p = sum(1 for c in CASES if c[2] == c[3])
    out.append(f'# Individual tests: {p} pass, {len(CASES) - p} fail, {len(NOTREP)} not representable')
    out.append('')
    for fl, what, exp, got, note in CASES:
        tag = 'pass' if exp == got else 'FAIL'
        out.append(f'{tag} {fl} | {what} | expected {exp} | got {got}' + (f' | {note}' if note else ''))
    out.append('')
    for fl, why in NOTREP:
        out.append(f'NOT REPRESENTABLE {fl} | {why}')
    return '\n'.join(out)


if __name__ == '__main__':
    print(main())
