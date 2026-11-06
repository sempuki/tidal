from score_tests import *  # noqa


def makes(w, src, target):
    """static_cast<T>(s): direct initialization (ICS-45)."""
    s = implicit_seq(w, pr(src), target, Ctx(direct=True))
    return s is not None and s.kind in ('standard', 'user')


def batch3():
    f = 'rank-class-value.cpp'
    w = World(); w.add_class('Base', ctors=[fn('Base()')]); w.add_class('Derived', bases=['Base'])
    case(f, 'take(d)', 'Two', call(w, [fn('One', cls('Base')), fn('Two', lref(const(cls('Derived'))))], [lv(cls('Derived'))]))
    w.add_class('Cell', ctors=[fn('Cell()')])
    Ce = cls('Cell')
    case(f, 'first::put(c, 0)', 'One', call(w, [fn('One', Ce, I), fn('Two', lref(const(Ce)), L)], [lv(Ce), pr(I)]))
    case(f, 'second::put(c, 0)', 'Two', call(w, [fn('One', Ce, L), fn('Two', lref(const(Ce)), I)], [lv(Ce), pr(I)]))
    f = 'rank-comparison.cpp'
    w = World()
    case(f, 'take(small)', 'One', call(w, [fn('One', I), fn('Two', L)], [lv(S)]))
    case(f, 'real(1.0f)', 'One', call(w, [fn('One', D), fn('Two', LD)], [pr(F)]))
    f = 'rank-worst-step.cpp'
    case(f, 'take(values)', 'One', call(w, [fn('One', ptr(const(I))), fn('Two', ptr(const(VOID)))], [lv(arr(I, 2))]))
    case(f, 'widen(small)', 'One', call(w, [fn('One', I), fn('Two', D)], [lv(S)]))

    f = 'reference-conversion-result.cpp'
    w = World(); w.add_class('Base'); w.add_class('Middle', bases=['Base'])
    w.add_class('Source', convs=[fn('op Middle&', ret=lref(cls('Middle')))])
    case(f, 'take(s)', 'Two', call(w, [fn('One', lref(cls('Base'))), fn('Two', lref(cls('Middle')))], [lv(cls('Source'))]))
    f = 'reference-conversion-value-result.cpp'
    w = World(); w.add_class('Item'); w.add_class('Base'); w.add_class('Derived', bases=['Base'])
    w.add_class('Source', convs=[fn('op Item', ret=cls('Item')), fn('op Derived', ret=cls('Derived'))])
    It, Bs, So = cls('Item'), cls('Base'), lv(cls('Source'))
    case(f, 'take(s)', 'One', call(w, [fn('One', rref(It)), fn('Two', lref(const(It)))], [So]))
    case(f, 'edit(s)', 'Two', call(w, [fn('One', lref(It)), fn('Two', ellipsis=True)], [So]))
    case(f, 'part(s)', 'One', call(w, [fn('One', rref(Bs)), fn('Two', lref(const(Bs)))], [So]))
    f = 'reference-direct-derived.cpp'
    w = World(); w.add_class('Base'); w.add_class('Derived', bases=['Base'])
    case(f, 'take(d)', 'Two', call(w, [fn('One', lref(const(cls('Base')))), fn('Two', cls('Derived'))], [lv(cls('Derived'))]))
    f = 'reference-direct-fewer-qualifiers.cpp'
    w = World()
    case(f, 'take(p)', 'One', call(w, [fn('One', lref(const(ptr(I)))), fn('Two', lref(const(ptr(const(I)))))], [lv(ptr(I))]))
    case(f, 'give(p)', 'One', call(w, [fn('One', lref(const(ptr(const(I))))), fn('Two', lref(const(ptr(cv(I)))))], [lv(ptr(I))]))
    f = 'reference-function-rvalue.cpp'
    case(f, 'take(run)', 'One', call(w, [fn('One', rref(fnty(VOID))), fn('Two', ellipsis=True)], [lv(fnty(VOID))]))
    f = 'reference-less-cv.cpp'
    case(f, 'take(value)', 'One', call(w, [fn('One', lref(I)), fn('Two', lref(const(I)))], [lv(I)]))
    case(f, 'look(value)', 'One', call(w, [fn('One', lref(const(I))), fn('Two', lref(cv(I)))], [lv(I)]))
    f = 'reference-no-sequence.cpp'
    case(f, 'change(1)', 'Two', call(w, [fn('One', lref(I)), fn('Two', ellipsis=True)], [pr(I)]))
    case(f, 'steal(value)', 'Two', call(w, [fn('One', rref(I)), fn('Two', ellipsis=True)], [lv(I)]))
    case(f, 'watch(1)', 'Two', call(w, [fn('One', lref(cv(I))), fn('Two', ellipsis=True)], [pr(I)]))
    f = 'reference-rvalue-other-type.cpp'
    w = World(); w.add_class('Base'); w.add_class('Derived', bases=['Base'])
    w.add_class('ToInt', convs=[fn('op int', ret=I)]); w.add_class('ToBase', convs=[fn('op Base&', ret=lref(cls('Base')))])
    w.add_enum('Color')
    tk = [fn('One', rref(I)), fn('Two', ellipsis=True)]
    case(f, 'take(s)', 'One', call(w, tk, [lv(S)]))
    case(f, 'take(c)', 'One', call(w, tk, [lv(enum('Color'))]))
    case(f, 'take(ti)', 'One', call(w, tk, [lv(cls('ToInt'))]))
    case(f, 'wide(i)', 'One', call(w, [fn('One', rref(const(L))), fn('Two', ellipsis=True)], [lv(I)]))
    pt = [fn('One', rref(cls('Base'))), fn('Two', ellipsis=True)]
    case(f, 'part(d)', 'Two', call(w, pt, [lv(cls('Derived'))]))
    case(f, 'part(tb)', 'Two', call(w, pt, [lv(cls('ToBase'))]))
    f = 'reference-temporary.cpp'
    w = World()
    case(f, 'take(small)', 'Two', call(w, [fn('One', lref(const(L))), fn('Two', lref(const(I)))], [lv(S)]))
    case(f, 'first::put(1, 0)', 'One', call(w, [fn('One', lref(const(L)), I), fn('Two', L, L)], [pr(I), pr(I)]))
    case(f, 'second::put(1, 0)', 'Two', call(w, [fn('One', lref(const(L)), L), fn('Two', L, I)], [pr(I), pr(I)]))
    f = 'rvalue-reference-before-cv.cpp'
    w = World(); w.add_class('Item')
    case(f, 'take(1)', 'One', call(w, [fn('One', rref(cv(I))), fn('Two', lref(const(I)))], [pr(I)]))
    case(f, 'put(Item{})', 'One', call(w, [fn('One', rref(cv(cls('Item')))), fn('Two', lref(const(cls('Item'))))], [pr(cls('Item'))]))
    f = 'rvalue-reference-before-qualification.cpp'
    case(f, 'take(&n)', 'Two', call(w, [fn('One', lref(const(ptr(const(I))))), fn('Two', rref(ptr(cv(I))))], [pr(ptr(I))]))
    f = 'rvalue-reference-binding.cpp'
    case(f, 'take(1)', 'One', call(w, [fn('One', rref(const(I))), fn('Two', lref(const(I)))], [pr(I)]))
    case(f, 'put(Item{})', 'One', call(w, [fn('One', rref(const(cls('Item')))), fn('Two', lref(const(cls('Item'))))], [pr(cls('Item'))]))
    f = 'rvalue-reference-object-exception.cpp'
    w = World(); w.add_class('Item'); w.add_class('Fixed')
    cs = [fn('One', I, kind='member', owner='Item', cv=k), fn('Two', rref(const(cls('Item'))), L)]
    case(f, 'Item{} + 0', 'One', call(w, cs, [pr(cls('Item')), pr(I)], has_object=True))
    cs = [fn('One', I, kind='member', owner='Fixed', cv=k, refq='&'), fn('Two', rref(const(cls('Fixed'))), I)]
    case(f, 'Fixed{} + 0', 'Two', call(w, cs, [pr(cls('Fixed')), pr(I)], has_object=True))
    f = 'standard-three-steps.cpp'
    w = World(); w.add_class('Base'); w.add_class('Derived', bases=['Base'])
    case(f, 'take(items)', 'One', call(w, [fn('One', ptr(const(cls('Base')))), fn('Two', ellipsis=True)], [lv(arr(cls('Derived'), 2))]))
    f = 'standard-top-level-cv.cpp'
    case(f, 'take(limit)', 'One', call(w, [fn('One', I), fn('Two', L)], [lv(const(I))]))
    case(f, 'keep(5)', 'One', call(w, [fn('One', const(I))], [pr(I)]))
    f = 'static-object-slot.cpp'
    w = World(); w.add_class('Tool'); w.add_class('Tool2')
    cs = [fn('One', L, I, kind='static', owner='Tool'), fn('Two', U, L, kind='member', owner='Tool')]
    case(f, 't.use(1, 0)', 'One', call(w, cs, [lv(cls('Tool')), pr(I), pr(I)], has_object=True))
    cs = [fn('One', L, L, kind='static', owner='Tool2'), fn('Two', U, I, kind='member', owner='Tool2')]
    case(f, 't2.use(1, 0)', 'Two', call(w, cs, [lv(cls('Tool2')), pr(I), pr(I)], has_object=True))
    f = 'static-object-slot-derived.cpp'
    w.add_class('Derived', bases=['Tool']); w.add_class('Derived2', bases=['Tool2'])
    cs = [fn('One', L, I, kind='static', owner='Tool'), fn('Two', L, L, kind='member', owner='Tool', cv=k)]
    case(f, 'd.use(1, 0)', 'One', call(w, cs, [lv(cls('Derived')), pr(I), pr(I)], has_object=True))
    cs = [fn('One', L, L, kind='static', owner='Tool2'), fn('Two', L, I, kind='member', owner='Tool2', cv=k)]
    case(f, 'd2.use(1, 0)', 'Two', call(w, cs, [lv(cls('Derived2')), pr(I), pr(I)], has_object=True))
    f = 'subsequence-function-pointer.cpp'
    w = World()
    case(f, 'take(run)', 'One', call(w, [fn('One', ptr(fnty(VOID, noexcept=True))), fn('Two', ptr(fnty(VOID)))], [lv(fnty(VOID, noexcept=True))]))
    f = 'subsequence-qualification.cpp'
    case(f, 'take(p)', 'One', call(w, [fn('One', ptr(I)), fn('Two', ptr(const(I)))], [lv(ptr(I))]))
    case(f, 'take(values)', 'One', call(w, [fn('One', ptr(I)), fn('Two', ptr(const(I)))], [lv(arr(I, 2))]))
    f = 'template-conversion-exact.cpp'
    w = World(); w.add_class('Source', convs=[fn('op int<T>', ret=I, template=True)]); w.add_class('Plain', convs=[fn('op int', ret=I)])
    case(f, 'take(Source{})', 'Two', call(w, [fn('One', L), fn('Two', ellipsis=True)], [pr(cls('Source'))]))
    case(f, 'keep(Source{})', 'One', call(w, [fn('One', I), fn('Two', ellipsis=True)], [pr(cls('Source'))]))
    case(f, 'take(Plain{})', 'One', call(w, [fn('One', L), fn('Two', ellipsis=True)], [pr(cls('Plain'))]))
    f = 'template-conversion-exact-long.cpp'
    w = World(); w.add_class('Source', convs=[fn('op long<T>', ret=L, template=True)])
    case(f, 'take(Source{})', 'One', call(w, [fn('One', L), fn('Two', ellipsis=True)], [pr(cls('Source'))]))
    case(f, 'scale(Source{})', 'Two', call(w, [fn('One', D), fn('Two', ellipsis=True)], [pr(cls('Source'))]))
    f = 'tie-breaker-before-rvalue-reference.cpp'
    w = World(); chain(w)
    Bs, M, Dv = cls('Base'), cls('Middle'), cls('Derived')
    case(f, 'flag(&n)', 'One', call(w, [fn('One', lref(const(ptr(VOID)))), fn('Two', rref(B))], [pr(ptr(I))]))
    case(f, 'near(Derived{})', 'One', call(w, [fn('One', lref(const(M))), fn('Two', rref(Bs))], [pr(Dv)]))
    case(f, 'base(&d)', 'One', call(w, [fn('One', lref(const(ptr(Bs)))), fn('Two', rref(ptr(VOID)))], [pr(ptr(Dv))]))
    case(f, 'nearer(&d)', 'One', call(w, [fn('One', lref(const(ptr(M)))), fn('Two', rref(ptr(Bs)))], [pr(ptr(Dv))]))
    case(f, 'member(&Base::x)', 'One', call(w, [fn('One', lref(const(mptr('Middle', I)))), fn('Two', rref(mptr('Derived', I)))], [pr(mptr('Base', I))]))
    f = 'two-user-defined-function-then-constructor.cpp'
    w = World(); w.add_class('Middle'); w.add_class('Source', convs=[fn('op Middle', ret=cls('Middle'))])
    w.add_class('Target', ctors=[fn('Target(Middle)', cls('Middle'))])
    tk = [fn('One', cls('Target')), fn('Two', ellipsis=True)]
    case(f, 'take(Source{})', 'Two', call(w, tk, [pr(cls('Source'))]))
    case(f, 'take(Middle{})', 'One', call(w, tk, [pr(cls('Middle'))]))
    f = 'two-user-defined-none.cpp'
    w = World(); w.add_class('Inner', ctors=[fn('Inner(int)', I)]); w.add_class('Outer', ctors=[fn('Outer(Inner)', cls('Inner'))])
    case(f, 'take(1)', 'Two', call(w, [fn('One', cls('Outer')), fn('Two', ellipsis=True)], [pr(I)]))

    f = 'user-defined-candidates.cpp'
    w = World(); w.add_class('Target'); w.add_class('Derived', bases=['Target'])
    w.add_class('Source', convs=[fn('op Derived', ret=cls('Derived'), cv=k)])
    case(f, 'Target t = Source{}', 'op Derived', conv_init(w, pr(cls('Source')), cls('Target')))
    w.add_class('Copyable', ctors=[fn('Copyable()')])
    w.add_class('Maker', convs=[fn('op Copyable', ret=cls('Copyable'), cv=k, explicit=True)])
    w.add_class('Address', convs=[fn('op int*', ret=ptr(I), cv=k, explicit=True)])
    w.add_class('Holder', convs=[fn('op Target&', ret=lref(cls('Target')), cv=k)])
    case(f, 'Copyable c(Maker{})', True, makes(w, cls('Maker'), cls('Copyable')))
    case(f, 'const int* p(Address{})', True, makes(w, cls('Address'), ptr(const(I))))
    case(f, 'Target& r = Holder{}', 'op Target&', conv_init(w, pr(cls('Holder')), lref(cls('Target'))))
    f = 'user-defined-choice-object-parameter.cpp'
    w = World()
    w.add_class('Target', ctors=[fn('Target()'), fn('Target(const Source&)', lref(const(cls('Source'))))])
    w.add_class('Source', convs=[fn('op Target', ret=cls('Target'))])
    w.add_class('Goal', ctors=[fn('Goal()'), fn('Goal(Maker&)', lref(cls('Maker')))])
    w.add_class('Maker', convs=[fn('op Goal', ret=cls('Goal'), cv=k)])
    case(f, 'Target t = s', 'op Target', conv_init(w, lv(cls('Source')), cls('Target')))
    case(f, 'take(s)', 'One', call(w, [fn('One', cls('Target')), fn('Two', ellipsis=True)], [lv(cls('Source'))]))
    case(f, 'Goal g = m', 'Goal(Maker&)', conv_init(w, lv(cls('Maker')), cls('Goal')))
    f = 'user-defined-different-functions.cpp'
    w = World(); w.add_class('Left', ctors=[fn('Left(int)', I)]); w.add_class('Right', ctors=[fn('Right(int)', I)])
    w.add_class('Source', convs=[fn('op int', ret=I), fn('op long', ret=L)])
    w.add_class('Point', aggregate=True, fields=[('x', I), ('y', I)]); w.add_class('Pair', ctors=[fn('Pair(int,int)', I, I)])
    Le, Ri, So, Po, Pa = cls('Left'), cls('Right'), pr(cls('Source')), cls('Point'), cls('Pair')
    case(f, 'first::take(1, 0)', 'One', call(w, [fn('One', Le, I), fn('Two', Ri, L)], [pr(I), pr(I)]))
    case(f, 'second::take(1, 0)', 'Two', call(w, [fn('One', Le, L), fn('Two', Ri, I)], [pr(I), pr(I)]))
    case(f, 'third::put(Source{}, 0)', 'One', call(w, [fn('One', I, I), fn('Two', L, L)], [So, pr(I)]))
    case(f, 'fourth::put(Source{}, 0)', 'Two', call(w, [fn('One', I, L), fn('Two', L, I)], [So, pr(I)]))
    case(f, 'fifth::set({1,2}, 0)', 'One', call(w, [fn('One', Po, I), fn('Two', Pa, L)], [braced(pr(I), pr(I)), pr(I)]))
    case(f, 'sixth::set({1,2}, 0)', 'Two', call(w, [fn('One', Po, L), fn('Two', Pa, I)], [braced(pr(I), pr(I)), pr(I)]))

    f = 'user-defined-explicit.cpp'
    w = World(); w.add_class('Base', ctors=[fn('Base()')]); w.add_class('Derived', bases=['Base'], ctors=[fn('Derived()')])
    w.add_class('Item', ctors=[fn('Item()')])
    It, Dv = cls('Item'), cls('Derived')
    for n, r in [('ToItem', It), ('ToConstItem', const(It)), ('ToItemRef', lref(It)), ('ToDerived', Dv),
                 ('ToDerivedRef', lref(Dv)), ('ToPointer', ptr(I)), ('ToIntRef', lref(I)), ('ToInt', I)]:
        w.add_class(n, convs=[fn('op ' + n, ret=r, explicit=True)])
    for src, tgt, exp in [('ToItem', It, True), ('ToConstItem', It, True), ('ToItemRef', It, True),
                          ('ToDerived', cls('Base'), False), ('ToPointer', ptr(const(I)), True),
                          ('ToPointer', ptr(const(VOID)), False), ('ToInt', L, False),
                          ('ToItemRef', lref(const(It)), True), ('ToIntRef', lref(const(I)), True),
                          ('ToDerivedRef', lref(cls('Base')), False)]:
        case(f, f'makes<{show(tgt)}, {src}>', exp, makes(w, cls(src), tgt))
    case(f, 'pass(ToItem{})', 'Two', call(w, [fn('One', It), fn('Two', ellipsis=True)], [pr(cls('ToItem'))]))
    f = 'user-defined-explicit-reference-other-type.cpp'
    w2 = World(); w2.add_class('Base', ctors=[fn('Base()')]); w2.add_class('Derived', bases=['Base'], ctors=[fn('Derived()')])
    for n, r in [('ToDerived', Dv), ('ToDerivedLvalue', lref(Dv)), ('ToDerivedRvalue', rref(Dv)),
                 ('ToInt', I), ('ToIntLvalue', lref(I)), ('ToIntRvalue', rref(I))]:
        w2.add_class(n, convs=[fn('op ' + n, ret=r, explicit=True)])
    for n in ['ToDerived', 'ToDerivedLvalue', 'ToDerivedRvalue']:
        for t in (rref(cls('Base')), lref(const(cls('Base')))):
            case(f, f'!makes<{show(t)}, {n}>', False, makes(w2, cls(n), t))
    for n in ['ToInt', 'ToIntLvalue', 'ToIntRvalue']:
        for t in (rref(L), lref(const(L))):
            case(f, f'!makes<{show(t)}, {n}>', False, makes(w2, cls(n), t))
    f = 'user-defined-explicit-reference-value.cpp'
    w3 = World(); w3.add_class('Item', ctors=[fn('Item()')])
    w3.add_class('ToItem', convs=[fn('op Item', ret=It, explicit=True)])
    w3.add_class('ToItemRvalue', convs=[fn('op Item&&', ret=rref(It), explicit=True)])
    w3.add_class('ToInt', convs=[fn('op int', ret=I, explicit=True)])
    w3.add_class('Base', ctors=[fn('Base()')]); w3.add_class('Derived', bases=['Base'], ctors=[fn('Derived()')])
    w3.add_class('ToDerived', convs=[fn('op Derived', ret=Dv, explicit=True)])
    case(f, '!makes<const Item&, ToItem>', False, makes(w3, cls('ToItem'), lref(const(It))))
    f = 'user-defined-explicit-reference-xvalue.cpp'
    case(f, 'makes<Item&&, ToItemRvalue>', True, makes(w3, cls('ToItemRvalue'), rref(It)))
    case(f, '!makes<const Item&, ToItemRvalue>', False, makes(w3, cls('ToItemRvalue'), lref(const(It))))
    f = 'user-defined-explicit-rvalue-reference-value.cpp'
    case(f, '!makes<long&&, ToInt>', False, makes(w3, cls('ToInt'), rref(L)))
    case(f, '!makes<Base&&, ToDerived>', False, makes(w3, cls('ToDerived'), rref(cls('Base'))))
    case(f, 'makes<Item&&, ToItem>', True, makes(w3, cls('ToItem'), rref(It)))
    case(f, 'makes<const Item&&, ToItem>', True, makes(w3, cls('ToItem'), rref(const(It))))
    case(f, 'makes<int&&, ToInt>', True, makes(w3, cls('ToInt'), rref(I)))

    f = 'user-defined-first-part.cpp'
    w = World(); w.add_class('Wide', ctors=[fn('Wide(long)', L)])
    case(f, 'take(1)', 'One', call(w, [fn('One', cls('Wide')), fn('Two', ellipsis=True)], [pr(I)]))
    f = 'user-defined-same-aggregate.cpp'
    w = World(); w.add_class('Point', aggregate=True, fields=[('x', I), ('y', I)]); w.add_class('Pair', ctors=[fn('Pair(int,int)', I, I)])
    case(f, 'take({1,2})', 'Two', call(w, [fn('One', lref(const(cls('Point')))), fn('Two', rref(cls('Point')))], [braced(pr(I), pr(I))]))
    case(f, 'put({1,2})', 'Two', call(w, [fn('One', lref(const(cls('Pair')))), fn('Two', rref(cls('Pair')))], [braced(pr(I), pr(I))]))
    f = 'user-defined-same-function.cpp'
    w = World(); w.add_class('Source', convs=[fn('op int', ret=I)]); w.add_class('Small', convs=[fn('op short', ret=S)])
    case(f, 'take(Source{})', 'Two', call(w, [fn('One', L), fn('Two', I)], [pr(cls('Source'))]))
    case(f, 'put(Small{})', 'One', call(w, [fn('One', I), fn('Two', L)], [pr(cls('Small'))]))
    f = 'user-defined-second-part.cpp'
    w = World(); w.add_class('Source', convs=[fn('op int', ret=I)]); w.add_class('Pointer', convs=[fn('op int*', ret=ptr(I))])
    case(f, 'take(Source{})', 'Two', call(w, [fn('One', lref(const(I))), fn('Two', rref(I))], [pr(cls('Source'))]))
    case(f, 'look(Pointer{})', 'One', call(w, [fn('One', ptr(const(I))), fn('Two', ellipsis=True)], [pr(cls('Pointer'))]))
    f = 'user-defined-step-standard-only.cpp'
    w = World(); w.add_class('Middle', convs=[fn('op int', ret=I, cv=k)]); w.add_class('Source', convs=[fn('op Middle', ret=cls('Middle'), cv=k)])
    w.add_class('Number', convs=[fn('op int', ret=I, cv=k)]); w.add_class('Box', ctors=[fn('Box(int)', I)])
    case(f, 'take(Source{})', 'Two', call(w, [fn('One', I), fn('Two', ellipsis=True)], [pr(cls('Source'))]))
    case(f, 'put(Number{})', 'Two', call(w, [fn('One', cls('Box')), fn('Two', ellipsis=True)], [pr(cls('Number'))]))


import score_tests
score_tests.BATCHES.append(batch3)
