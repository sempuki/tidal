from score_tests import *  # noqa


def batch4():
    w = World()
    f = 'best-beats-each.cpp'
    case(f, 'f(1, 1)', 'One', call(w, [fn('Two a', L, L), fn('One', I, I), fn('Two b', I, L), fn('Two c', L, I)], [pr(I), pr(I)]))
    f = 'best-no-single-winner.cpp'
    case(f, 'f(1, 1)', 'ambiguous', call(w, [fn('a', I, L), fn('b', L, L), fn('c', L, I)], [pr(I), pr(I)]))
    f = 'better-crossing-arguments.cpp'
    case(f, 'f(1, 1)', 'ambiguous', call(w, [fn('a', I, L), fn('c', L, I)], [pr(I), pr(I)]))
    f = 'better-one-argument.cpp'
    case(f, 'f(1, 1)', 'One', call(w, [fn('One', I, L), fn('Two', L, L)], [pr(I), pr(I)]))
    f = 'template-exact-over-plain.cpp'
    case(f, 'f(1L)', 'Two', call(w, [fn('One', I), fn('Two', L, template=True)], [pr(L)]))

    # Constraints and partial ordering are the caller's black boxes.
    f = 'tie-constraints-default-parameter.cpp'
    a, b = fn('Known', I), fn('Small', I, I, defaults=1)
    case(f, 'Pick<int>::f(1)', 'ambiguous', call(w, [a, b], [pr(I)], more_constrained=lambda x, y: False),
         'the caller answers that ORD-13 conditions fail (parameter lists differ)')
    f = 'tie-constraints-ellipsis.cpp'
    a, b = fn('Known', I), fn('Small', I, ellipsis=True)
    case(f, 'Pick<int>::f(1)', 'ambiguous', call(w, [a, b], [pr(I)], more_constrained=lambda x, y: False))
    f = 'tie-constraints-different-parameters.cpp'
    a, b = fn('const T&', lref(const(I)), template=True), fn('T', I, template=True)
    case(f, 'f(n)', 'ambiguous', call(w, [a, b], [lv(I)], order=lambda x, y: None,
                                       more_constrained=lambda x, y: x is b))
    f = 'tie-conversion-result-before-constraints.cpp'
    w1 = World(); w1.add_class('Source', convs=[fn('op int', ret=I, cv=k), fn('op long', ret=L, cv=k)])
    case(f, 'int x = Source<int>{}', 'op int', conv_init(w1, pr(cls('Source')), I))
    f = 'tie-conversion-result-before-template.cpp'
    w1 = World(); w1.add_class('Source', convs=[fn('op long', ret=L, cv=k), fn('op T<int>', ret=I, cv=k, template=True)])
    case(f, 'int n = Source{}', 'op T<int>', conv_init(w1, pr(cls('Source')), I))
    f = 'tie-conversion-result-constructor.cpp'
    w1 = World()
    w1.add_class('Target', ctors=[fn('Target(const Source&)', lref(const(cls('Source')))), fn('Target(int)', I)])
    w1.add_class('Derived', bases=['Target'], ctors=[fn('Derived()')])
    w1.add_class('Source', convs=[fn('op Derived', ret=cls('Derived'), cv=k)])
    case(f, 'Target t = s', 'Target(const Source&)', conv_init(w1, lv(const(cls('Source'))), cls('Target')))
    f = 'tie-conversion-result.cpp'
    w1 = World(); w1.add_class('Source', convs=[fn('op int', ret=I, cv=k), fn('op long', ret=L, cv=k)])
    case(f, 'long a = Source{}', 'op long', conv_init(w1, pr(cls('Source')), L))
    case(f, 'int b = Source{}', 'op int', conv_init(w1, pr(cls('Source')), I))
    Plain, Safe = ptr(fnty(I)), ptr(fnty(I, noexcept=True))
    f = 'tie-conversion-result-noexcept.cpp'
    w1 = World(); w1.add_class('Source', convs=[fn('op Plain', ret=Plain, cv=k), fn('op Safe', ret=Safe, cv=k)])
    case(f, 'int (*p)() = Source{}', 'op Plain', conv_init(w1, pr(cls('Source')), Plain))
    f = 'tie-conversion-result-templates.cpp'
    w1 = World(); w1.add_class('Source', convs=[fn('op fp<T>', ret=Plain, cv=k, template=True), fn('op nfp<T>', ret=Safe, cv=k, template=True)])
    case(f, 'int (*p)() = Source{}', 'op fp<T>', conv_init(w1, pr(cls('Source')), Plain))
    f = 'tie-derived-over-inherited.cpp'
    w1 = World(); w1.add_class('Base'); w1.add_class('Derived', bases=['Base'])
    cs = [fn('Derived(int,int=0)', I, I, defaults=1, kind='ctor', owner='Derived'),
          fn('Base(int) inherited', I, kind='ctor', owner='Derived', inherited=True)]
    case(f, 'Derived(5)', 'Derived(int,int=0)', call(w1, cs, [pr(I)]))
    Fty = fnty(I)
    f = 'tie-function-reference-kind.cpp'
    w1 = World(); w1.add_class('Source', convs=[fn('op F&', ret=lref(Fty), cv=k), fn('op F&&', ret=rref(Fty), cv=k)])
    case(f, 'F& left = Source{}', 'op F&', conv_init(w1, pr(cls('Source')), lref(Fty)))
    case(f, 'F&& right = Source{}', 'op F&&', conv_init(w1, pr(cls('Source')), rref(Fty)))
    f = 'tie-more-constrained.cpp'
    w1 = World(); w1.add_class('Box')
    a = fn('One', kind='member', owner='Box'); b = fn('Two', kind='member', owner='Box')
    case(f, 'Box<int>{}.get()', 'Two', call(w1, [a, b], [pr(cls('Box'))], has_object=True, more_constrained=lambda x, y: x is b))
    b2 = fn('Two', kind='member', owner='Box', constraints_ok=False)
    case(f, 'Box<double>{}.get()', 'One', call(w1, [a, b2], [pr(cls('Box'))], has_object=True))
    f = 'tie-more-specialized-constraints.cpp'
    a, b = fn('One', I, template=True), fn('Two', I, template=True)
    case(f, 'f(1)', 'Two', call(w, [a, b], [pr(I)], order=lambda x, y: 'equal', more_constrained=lambda x, y: x is b))
    a, b = fn('One', C, template=True), fn('Two', C, template=True, constraints_ok=False)
    case(f, "f('c')", 'One', call(w, [a, b], [pr(C)], order=lambda x, y: 'equal', more_constrained=lambda x, y: x is b))
    f = 'tie-more-specialized.cpp'
    a, b = fn('One', ptr(I), template=True), fn('Two', ptr(I), template=True)
    case(f, 'f(p)', 'Two', call(w, [a, b], [lv(ptr(I))], order=lambda x, y: 'first' if x is b else 'second'))
    f = 'tie-plain-over-template.cpp'
    case(f, 'f(1)', 'One', call(w, [fn('One', I), fn('Two', I, template=True)], [pr(I)]))
    f = 'tie-reference-kind-before-constraints.cpp'
    w1 = World(); w1.add_class('Source', convs=[fn('op F&', ret=lref(Fty), cv=k), fn('op F&&', ret=rref(Fty), cv=k)])
    case(f, 'F& r = Source<int>{}', 'op F&', conv_init(w1, pr(cls('Source')), lref(Fty)),
         'constraints cannot reach a nested conversion in this API; OVL-29 decides first anyway')
    f = 'tie-reference-kind-before-more-specialized.cpp'
    w1 = World(); w1.add_class('Source', convs=[fn('op T&', ret=lref(Fty), cv=k, template=True), fn('op T&&', ret=rref(Fty), cv=k, template=True)])
    case(f, 'F&& r = Source{}', 'op T&&', conv_init(w1, pr(cls('Source')), rref(Fty)))
    f = 'tie-reference-kind-before-template.cpp'
    w1 = World(); w1.add_class('Source', convs=[fn('op F&', ret=lref(Fty), cv=k), fn('op T&&', ret=rref(Fty), cv=k, template=True)])
    case(f, 'F&& r = Source{}', 'op T&&', conv_init(w1, pr(cls('Source')), rref(Fty)))
    f = 'tie-reference-kind-before-template-lvalue.cpp'
    w1 = World(); w1.add_class('Source', convs=[fn('op F&&', ret=rref(Fty), cv=k), fn('op T&', ret=lref(Fty), cv=k, template=True)])
    case(f, 'F& r = Source{}', 'op T&', conv_init(w1, pr(cls('Source')), lref(Fty)))
    f = 'tie-swapped-same-function.cpp'
    w1 = World(); w1.add_class('Item'); It = cls('Item')
    cs = [fn('written', lref(const(It)), kind='member', owner='Item'),
          fn('swapped', lref(const(It)), lref(It), rewritten=True, swapped=True)]
    case(f, 'a == b', 'ambiguous', call(w1, cs, [lv(It), lv(It)], has_object=True),
         'the swapped member candidate is modelled as a free function with its parameters reversed')
    f = 'tie-template-before-inherited.cpp'
    w1 = World(); w1.add_class('Base'); w1.add_class('Derived', bases=['Base'])
    cs = [fn('Derived(T,int=0)', I, I, defaults=1, kind='ctor', owner='Derived', template=True),
          fn('Base(int) inherited', I, kind='ctor', owner='Derived', inherited=True)]
    case(f, 'Derived(5)', 'Base(int) inherited', call(w1, cs, [pr(I)]))
    f = 'tie-template-before-rewritten.cpp'
    w1 = World(); w1.add_class('Item'); It = cls('Item')
    cs = [fn('written !=<T>', lref(const(It)), lref(const(It)), template=True),
          fn('rewritten ==', lref(const(It)), lref(const(It)), rewritten=True),
          fn('swapped ==', lref(const(It)), lref(const(It)), rewritten=True, swapped=True)]
    case(f, 'Item{} != Item{}', 'rewritten ==', call(w1, cs, [pr(It), pr(It)]))
    f = 'tie-template-before-swapped.cpp'
    w1 = World(); w1.add_class('Left'); w1.add_class('Right')
    cs = [fn('written ==<T>', lref(const(cls('Left'))), lref(const(cls('Right'))), template=True),
          fn('swapped ==', lref(const(cls('Left'))), lref(const(cls('Right'))), rewritten=True, swapped=True)]
    case(f, 'Left{} == Right{}', 'swapped ==', call(w1, cs, [pr(cls('Left')), pr(cls('Right'))]))
    f = 'tie-written-order.cpp'
    w1 = World(); w1.add_class('Item'); It = cls('Item')
    cs = [fn('One', lref(const(It)), lref(const(It)), rewritten=True),
          fn('Two', lref(const(It)), lref(const(It)), rewritten=True, swapped=True)]
    case(f, 'a < b', 'One', call(w1, cs, [lv(It), lv(It)]))
    f = 'tie-written-over-rewritten.cpp'
    cs = [fn('One', lref(const(It)), kind='member', owner='Item', cv=k),
          fn('rewritten', lref(const(It)), kind='member', owner='Item', cv=k, rewritten=True),
          fn('swapped', lref(const(It)), lref(const(It)), rewritten=True, swapped=True)]
    case(f, 'a < b', 'One', call(w1, cs, [lv(It), lv(It)], has_object=True))


import score_tests
score_tests.BATCHES.append(batch4)
