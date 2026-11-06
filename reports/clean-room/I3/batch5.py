from score_tests import *  # noqa
from score import PROMOTED_ARITH


def batch5():
    f = 'init-constructor-against-conversion.cpp'
    w = World()
    w.add_class('Target', ctors=[fn('Target(Source&)', lref(cls('Source'))), fn('Target(int)', I)])
    w.add_class('Source', convs=[fn('op Target', ret=cls('Target'), cv=k)])
    case(f, 'Target t = s', 'Target(Source&)', conv_init(w, lv(cls('Source')), cls('Target')))
    f = 'init-constructor-conversion-tie.cpp'
    w = World()
    w.add_class('Target', ctors=[fn('Target(const Source&)', lref(const(cls('Source'))))])
    w.add_class('Source', convs=[fn('op Target', ret=cls('Target'), cv=k)])
    case(f, 'Target t = s', 'ambiguous', conv_init(w, lv(cls('Source')), cls('Target')))
    f = 'init-conversion-to-derived.cpp'
    w = World(); w.add_class('Base'); w.add_class('Derived', bases=['Base'], ctors=[fn('Derived()')])
    w.add_class('Source', convs=[fn('op Derived', ret=cls('Derived'), cv=k)])
    case(f, 'Base b = Source{}', 'op Derived', conv_init(w, pr(cls('Source')), cls('Base')))
    f = 'init-copy-constructor-standard-only.cpp'
    w = World(); w.add_class('Mid'); w.add_class('Target', ctors=[fn('Target(Mid)', cls('Mid'))])
    w.add_class('ToMid', convs=[fn('op Mid', ret=cls('Mid'), cv=k)])
    case(f, 'Target copied = ToMid{}', 'none', conv_init(w, pr(cls('ToMid')), cls('Target')))
    case(f, 'Target direct(ToMid{})', 'Target(Mid)', call(w, w.classes['Target'].ctors, [pr(cls('ToMid'))]))
    f = 'init-copy-explicit-conversion.cpp'
    w = World(); w.add_class('Target', ctors=[fn('Target(int)', I)])
    w.add_class('Source', convs=[fn('op Target', ret=cls('Target'), cv=k, explicit=True)])
    case(f, 'Target t = s', 'none', conv_init(w, lv(cls('Source')), cls('Target')))
    f = 'init-direct-explicit-conversion.cpp'
    case(f, 'Target t(s)', 'Target(const Target&)', call(w, w.classes['Target'].ctors, [lv(cls('Source'))]),
         'resolve has no direct-initialization flag for the copy constructor parameter (OVL-40)')
    f = 'init-copy-list-explicit-conversion.cpp'
    w = World(); w.add_class('Target', ctors=[fn('Target()')])
    w.add_class('Source', convs=[fn('op Target', ret=cls('Target'), cv=k, explicit=True)])
    s = implicit_seq(w, braced(lv(cls('Source'))), cls('Target'))
    case(f, 'Target t = {s}', 'none', 'none' if s is None else s.kind)
    f = 'init-qualified-result.cpp'
    w = World(); w.add_class('Box', aggregate=True, fields=[('v', I)])
    w.add_class('Source', convs=[fn('op const Box&', ret=lref(const(cls('Box'))), cv=k)])
    case(f, 'Box b = Source{}', 'op const Box&', conv_init(w, pr(cls('Source')), cls('Box')))

    f = 'reference-conversion-kind.cpp'
    w = World(); w.add_class('Source', convs=[fn('op int&', ret=lref(I), cv=k), fn('op int&&', ret=rref(I), cv=k)])
    case(f, 'int& left = Source{}', 'op int&', conv_init(w, pr(cls('Source')), lref(I)))
    case(f, 'int&& right = Source{}', 'op int&&', conv_init(w, pr(cls('Source')), rref(I)))
    f = 'reference-conversion-not-compatible.cpp'
    w = World(); w.add_class('Source', convs=[fn('op long&', ret=lref(L), cv=k)])
    s = implicit_seq(w, pr(cls('Source')), lref(const(I)))
    case(f, 'const int& r = Source{} binds a temporary', 'user/temporary',
         'user/temporary' if s and s.kind == 'user' and s.third.ref and not s.third.ref.direct else repr(s))
    f = 'reference-explicit-copy.cpp'
    w = World(); w.add_class('Source', convs=[fn('op int&', ret=lref(I), cv=k, explicit=True)])
    case(f, 'const int& r = s', 'none', conv_init(w, lv(cls('Source')), lref(const(I))))
    f = 'reference-explicit-direct.cpp'
    s = implicit_seq(w, lv(cls('Source')), lref(const(I)), Ctx(direct=True))
    case(f, 'const int& r(s)', 'op int&', s.func.name if s and s.kind == 'user' else repr(s))
    f = 'reference-function-both-kinds.cpp'
    Fty = fnty(I)
    w = World(); w.add_class('Source', convs=[fn('op F&&', ret=rref(Fty), cv=k)])
    case(f, 'F& r = Source{}', 'op F&&', conv_init(w, pr(cls('Source')), lref(Fty)))

    f = 'viable-constraints.cpp'
    w = World(); w.add_class('Box')
    cs = [fn('One', D, kind='member', owner='Box'), fn('Two', LD, kind='member', owner='Box')]
    case(f, 'Box<double>{}.put(1.0)', 'One', call(w, cs, [pr(cls('Box')), pr(D)], has_object=True))
    cs = [fn('One', C, kind='member', owner='Box', constraints_ok=False), fn('Two', LD, kind='member', owner='Box')]
    case(f, "Box<char>{}.put('a')", 'Two', call(w, cs, [pr(cls('Box')), pr(C)], has_object=True))
    f = 'viable-constraints-function-template.cpp'
    case(f, 'put(1.0)', 'One', call(w, [fn('One', D, template=True), fn('Two', LD)], [pr(D)]))
    case(f, "put('a')", 'Two', call(w, [fn('One', C, template=True, constraints_ok=False), fn('Two', LD)], [pr(C)]))
    f = 'viable-count.cpp'
    fs = lambda: [fn('One', I, I), fn('Two', I, I, I, I, defaults=1), fn('Three', I, ellipsis=True)]
    for n, exp in [(2, 'One'), (3, 'Two'), (5, 'Three'), (1, 'Three')]:
        case(f, f'f with {n} arguments', exp, call(w, fs(), [pr(I)] * n))
    for n in (0, 3):
        case(f, f'g with {n} arguments', 'One', call(w, [fn('One', *([I] * n), template=True), fn('Two', I, I)], [pr(I)] * n))
    f = 'viable-count-defaults-and-ellipsis.cpp'
    for n, exp in [(1, 'One'), (3, 'One'), (4, 'Two'), (6, 'Two')]:
        case(f, f'k with {n} arguments', exp, call(w, [fn('One', I, I, I, defaults=2), fn('Two', I, I, I, I, ellipsis=True)], [pr(I)] * n))
    f = 'viable-sequence.cpp'
    w = World(); w.add_class('Box', ctors=[fn('Box(int)', I)]); w.add_class('Item')
    cs = [fn('One', lref(I), L), fn('Two', lref(const(L)), cls('Box'))]
    case(f, 'f(n, 1)', 'One', call(w, cs, [lv(I), pr(I)]))
    case(f, 'f(1, 1)', 'Two', call(w, cs, [pr(I), pr(I)]))
    cs = [fn('One', L, kind='member', owner='Item', refq='&'), fn('Two', D, kind='member', owner='Item', refq='&&')]
    case(f, 'v.get(1)', 'One', call(w, cs, [lv(cls('Item')), pr(I)], has_object=True))
    case(f, 'Item{}.get(1)', 'Two', call(w, cs, [pr(cls('Item')), pr(I)], has_object=True))
    f = 'defaults-take-no-part.cpp'
    case(f, 'f(1)', 'ambiguous', call(w, [fn('a', I), fn('b', I, I, defaults=1)], [pr(I)]))
    f = 'static-slot-no-part.cpp'
    cs = [fn('One', I, kind='static', owner='Item'), fn('Two', L, kind='member', owner='Item', cv=k)]
    case(f, 'v.get(1)', 'One', call(w, cs, [lv(cls('Item')), pr(I)], has_object=True))
    case(f, 'v.get(1L)', 'Two', call(w, cs, [lv(cls('Item')), pr(L)], has_object=True))
    f = 'no-object-nonstatic-chosen.cpp'
    cs = [fn('reset()', kind='member', owner='Item'), fn('reset(int)', I, kind='static', owner='Item')]
    case(f, 'reset() chooses the non-static member', 'reset()', call(w, cs, []))
    f = 'no-object-static-chosen.cpp'
    case(f, 'reset(0)', 'reset(int)', call(w, cs, [pr(I)]))
    f = 'no-object-ref-qualified-member.cpp'
    w.add_class('Store')
    cs = [fn('put(long)', L, kind='static', owner='Store'), fn('put(int)&&', I, kind='member', owner='Store', refq='&&')]
    case(f, 'put(1) chooses put(int)&&', 'put(int)&&', call(w, cs, [pr(I)]))
    f = 'no-object-static-default-ambiguous.cpp'
    cs = [fn('reset(int=0)', I, defaults=1, kind='static', owner='Item'), fn('reset()', kind='member', owner='Item')]
    case(f, 'reset()', 'ambiguous', call(w, cs, []))

    f = 'builtin-logical-explicit-bool.cpp'
    w = World(); w.add_class('Flag', convs=[fn('op bool', ret=B, cv=k, explicit=True)])
    case(f, '!f', 'builtin !', call(w, [fn('builtin !', B, kind='builtin')], [lv(cls('Flag'))]),
         'no per-parameter direct-initialization flag (ICS-45, OVL-9)')
    f = 'builtin-logical-explicit-competes.cpp'
    w.add_class('Gate', ctors=[fn('Gate(Flag)', cls('Flag'))])
    case(f, '!f', 'ambiguous', call(w, [fn('builtin !', B, kind='builtin'), fn('operator!(Gate)', cls('Gate'))], [lv(cls('Flag'))]))
    f = 'builtin-logical-explicit-competes-and.cpp'
    case(f, 'f && f', 'ambiguous', call(w, [fn('builtin &&', B, B, kind='builtin'), fn('operator&&(Gate,Gate)', cls('Gate'), cls('Gate'))],
                                        [lv(cls('Flag')), lv(cls('Flag'))]))
    f = 'builtin-logical-explicit-int.cpp'
    w = World(); w.add_class('Count', convs=[fn('op int', ret=I, cv=k, explicit=True)])
    case(f, '!c', 'no viable function', call(w, [fn('builtin !', B, kind='builtin')], [lv(cls('Count'))]))

    ARITH_NOT_BOOL = [x for x in INTEGRAL + FLOATING if x != 'bool']
    incs = lambda: [fn(f'builtin ++({q}{x}&)', lref(cq(fund(x), *q.split())), kind='builtin')
                    for x in ARITH_NOT_BOOL for q in ('', 'volatile ')]
    f = 'builtin-increment-from-conversion.cpp'
    w = World(); w.add_class('Cell', convs=[fn('op int&', ret=lref(I))])
    case(f, '++c', 'builtin ++(int&)', call(w, incs(), [lv(cls('Cell'))]))
    cas = [fn(f'builtin +=({x}&,{y})', lref(fund(x)), fund(y), kind='builtin') for x in ARITH_NOT_BOOL for y in PROMOTED_ARITH]
    case(f, 'c += 5', 'builtin +=(int&,int)', call(w, cas, [lv(cls('Cell')), pr(I)]))
    f = 'builtin-increment-from-value.cpp'
    w = World(); w.add_class('Count', convs=[fn('op int', ret=I, cv=k)])
    case(f, '++c', 'no viable function', call(w, incs(), [lv(cls('Count'))]))
    f = 'builtin-other-shape-kept.cpp'
    w = World(); w.add_enum('Color')
    Co = enum('Color')
    cs = [fn(f'builtin ==({a},{b})', fund(a), fund(b), kind='builtin') for a in PROMOTED_ARITH for b in PROMOTED_ARITH]
    cs += [fn('builtin ==(Color,Color)', Co, Co, kind='builtin'), fn('operator==(const Color&,const Color&)', lref(const(Co)), lref(const(Co)))]
    case(f, 'red == blue', 'ambiguous', call(w, cs, [pr(Co), pr(Co)]))


import score_tests
score_tests.BATCHES.append(batch5)
