from score_tests import *  # noqa


def lit(n, ch='char'):
    return Arg(arr(fund(ch, {'const'}), n + 1), 'lvalue', string=True)


def batch2():
    f = 'implicit-object-standard-only.cpp'
    w = World(); w.add_class('Item', ctors=[fn('Item()')])
    cs = [fn('One', I, kind='member', owner='Item'), fn('Two', L, kind='member', owner='Item', cv=k)]
    case(f, 'fixed.get(1)', 'Two', call(w, cs, [lv(const(cls('Item'))), pr(I)], has_object=True))

    f = 'initializer-list-over-kind.cpp'
    w = World(); w.add_class('Num', ctors=[fn('Num(int)', I)])
    case(f, 'take({1})', 'Two', call(w, [fn('One', ilist(cls('Num'))), fn('Two', I)], [braced(pr(I))]))
    f = 'initializer-list-over-rank.cpp'
    case(f, 'take({1})', 'One', call(World(), [fn('One', ilist(L)), fn('Two', I)], [braced(pr(I))]))
    f = 'initializer-list-reference.cpp'
    w = World()
    case(f, 'take({1})', 'Two', call(w, [fn('One', I), fn('Two', lref(const(ilist(L))))], [braced(pr(I))]))
    case(f, 'wide({1})', 'Two', call(w, [fn('One', L), fn('Two', lref(const(ilist(I))))], [braced(pr(I))]))
    case(f, 'move({1})', 'One', call(w, [fn('One', rref(ilist(I))), fn('Two', I)], [braced(pr(I))]))
    f = 'initializer-list-target.cpp'
    case(f, 'take({1})', 'One', call(w, [fn('One', ilist(I)), fn('Two', I)], [braced(pr(I))]))

    f = 'kind-order.cpp'
    w = World(); w.add_class('Box', ctors=[fn('Box(int)', I)])
    case(f, 'take(1)', 'One', call(w, [fn('One', L), fn('Two', cls('Box'))], [pr(I)]))
    case(f, 'put(1)', 'One', call(w, [fn('One', cls('Box')), fn('Two', ellipsis=True)], [pr(I)]))
    case(f, 'any(1)', 'One', call(w, [fn('One', L), fn('Two', ellipsis=True)], [pr(I)]))

    f = 'list-aggregate.cpp'
    w = World(); w.add_class('Point', aggregate=True, fields=[('x', I)])
    w.add_class('Derived', bases=['Point'], aggregate=True, fields=[])
    w.add_class('Wrap', ctors=[fn('Wrap(Point)', cls('Point'))])
    P = cls('Point')
    case(f, 'take({1})', 'Two', call(w, [fn('One', P), fn('Two', L)], [braced(pr(I))]))
    case(f, 'copy({p})', 'One', call(w, [fn('One', P), fn('Two', cls('Wrap'))], [braced(lv(P))]))
    case(f, 'base({d})', 'One', call(w, [fn('One', P), fn('Two', ellipsis=True)], [braced(lv(cls('Derived')))]))
    f = 'list-aggregate-nearest-base.cpp'
    w = World(); w.add_class('Base', aggregate=True, fields=[('x', I)])
    w.add_class('Middle', bases=['Base'], aggregate=True); w.add_class('Derived', bases=['Middle'], aggregate=True)
    case(f, 'take({d})', 'One', call(w, [fn('One', cls('Middle')), fn('Two', cls('Base'))], [braced(lv(cls('Derived')))]))

    f = 'list-array-against-scalar.cpp'
    case(f, 'take({1}, 0)', 'Two', call(World(), [fn('One', I, L), fn('Two', rref(arr(I, 2)), I)], [braced(pr(I)), pr(I)]))
    f = 'list-array.cpp'
    w = World()
    case(f, 'take({1})', 'One', call(w, [fn('One', rref(arr(I, 1))), fn('Two', L)], [braced(pr(I))]))
    case(f, 'open({1,2})', 'One', call(w, [fn('One', rref(arr(I))), fn('Two', ellipsis=True)], [braced(pr(I), pr(I))]))
    case(f, 'room({1,2})', 'One', call(w, [fn('One', rref(arr(I, 3))), fn('Two', L)], [braced(pr(I), pr(I))]))
    case(f, 'text({"ab"})', 'One', call(w, [fn('One', lref(arr(const(C), 3))), fn('Two', ellipsis=True)], [braced(lit(2))]))
    case(f, 'full({1,2})', 'Two', call(w, [fn('One', rref(arr(I, 1))), fn('Two', rref(arr(I, 2)))], [braced(pr(I), pr(I))]))

    f = 'list-class-constructor.cpp'
    w = World(); w.add_class('Box', ctors=[fn('Box(int)', I)]); w.add_class('Num', ctors=[fn('Num(int)', I)])
    w.add_class('Pair', ctors=[fn('Pair(Num,Num)', cls('Num'), cls('Num'))])
    case(f, 'take({1})', 'Two', call(w, [fn('One', cls('Box')), fn('Two', L)], [braced(pr(I))]))
    case(f, 'put({1,2})', 'One', call(w, [fn('One', cls('Pair')), fn('Two', ellipsis=True)], [braced(pr(I), pr(I))]))

    f = 'list-class-list-constructor.cpp'
    w = World()
    w.add_class('Cell', ctors=[fn('Cell()'), fn('Cell(il<int>)', ilist(I))], convs=[fn('op int', ret=I, cv=k)])
    w.add_class('Wrap', ctors=[fn('Wrap(Cell)', cls('Cell'))])
    case(f, 'take({c}, 0)', 'Two', call(w, [fn('One', cls('Cell'), L), fn('Two', cls('Wrap'), I)], [braced(lv(cls('Cell'))), pr(I)]))

    f = 'list-class-nearest-base.cpp'
    w = World()
    w.add_class('Base', ctors=[fn('Base()')]); w.add_class('Middle', bases=['Base'], ctors=[fn('Middle()')])
    w.add_class('Derived', bases=['Middle'], ctors=[fn('Derived()')])
    case(f, 'take({d})', 'One', call(w, [fn('One', cls('Middle')), fn('Two', cls('Base'))], [braced(lv(cls('Derived')))]))

    f = 'list-class-same-class.cpp'
    w = World(); w.add_class('Cell', ctors=[fn('Cell()'), fn('Cell(int)', I)])
    w.add_class('Wrap', ctors=[fn('Wrap(Cell)', cls('Cell'))]); w.add_class('Derived', bases=['Cell'], ctors=[fn('Derived()')])
    w.add_class('Holder', ctors=[fn('Holder(const Derived&)', lref(const(cls('Derived'))))])
    Ce, Dd = cls('Cell'), cls('Derived')
    case(f, 'take({c})', 'One', call(w, [fn('One', Ce), fn('Two', cls('Wrap'))], [braced(lv(Ce))]))
    case(f, 'base({d})', 'One', call(w, [fn('One', Ce), fn('Two', cls('Holder'))], [braced(lv(Dd))]))
    case(f, 'rank({d})', 'One', call(w, [fn('One', Ce), fn('Two', L)], [braced(lv(Dd))]))

    f = 'list-class-tie.cpp'
    w = World(); w.add_class('Split', ctors=[fn('Split(int,long)', I, L), fn('Split(long,int)', L, I)])
    w.add_class('Even', ctors=[fn('Even(int,int)', I, I)])
    case(f, 'take({1,1}, 0)', 'Two', call(w, [fn('One', cls('Split'), L), fn('Two', cls('Even'), I)], [braced(pr(I), pr(I)), pr(I)]))

    f = 'list-designated.cpp'
    w = World(); w.add_class('Point', aggregate=True, fields=[('x', I), ('y', I)])
    w.add_class('Swapped', aggregate=True, fields=[('y', I), ('x', I)]); w.add_class('Other', aggregate=True, fields=[('z', I)])
    case(f, 'take({.x=1,.y=2}, 0)', 'One', call(w, [fn('One', cls('Point'), I), fn('Two', cls('Swapped'), L)],
                                              [braced(pr(I), pr(I), designators=['x', 'y']), pr(I)]))
    case(f, 'pick({.x=1})', 'One', call(w, [fn('One', cls('Point')), fn('Two', cls('Other'))], [braced(pr(I), designators=['x'])]))

    f = 'list-initializer-list.cpp'
    w = World(); w.add_class('Num', ctors=[fn('Num(int)', I)])
    case(f, 'take({small, 1})', 'One', call(w, [fn('One', ilist(I)), fn('Two', ilist(L))], [braced(lv(S), pr(I))]))
    case(f, 'put({1, 2})', 'Two', call(w, [fn('One', ilist(cls('Num'))), fn('Two', ilist(L))], [braced(pr(I), pr(I))]))
    case(f, 'none({})', 'One', call(w, [fn('One', ilist(I)), fn('Two', ellipsis=True)], [braced()]))
    f = 'list-initializer-list-designated.cpp'
    w = World(); w.add_class('Point', aggregate=True, fields=[('x', I)])
    case(f, 'take({.x=1})', 'Two', call(w, [fn('One', ilist(I)), fn('Two', cls('Point'))], [braced(pr(I), designators=['x'])]))

    f = 'list-reference-class-element-class.cpp'
    w = World(); w.add_class('Item', ctors=[fn('Item()')]); w.add_class('Source', convs=[fn('op Item&', ret=lref(cls('Item')))])
    It = cls('Item')
    case(f, 'edit({s})', 'Two', call(w, [fn('One', lref(It)), fn('Two', lref(const(It)))], [braced(lv(cls('Source')))]))
    f = 'list-reference-class-element-scalar.cpp'
    w = World(); w.add_class('Source', convs=[fn('op int&', ret=lref(I))])
    case(f, 'edit({s})', 'Two', call(w, [fn('One', lref(I)), fn('Two', L)], [braced(lv(cls('Source')))]))
    f = 'list-reference.cpp'
    w = World()
    case(f, 'take({1})', 'One', call(w, [fn('One', lref(const(L))), fn('Two', ellipsis=True)], [braced(pr(I))]))
    case(f, 'put({1})', 'Two', call(w, [fn('One', lref(const(L))), fn('Two', lref(const(I)))], [braced(pr(I))]))
    f = 'list-reference-direct.cpp'
    w = World(); w.add_class('Base', aggregate=True, fields=[('x', I)]); w.add_class('Derived', bases=['Base'], aggregate=True)
    Bs = cls('Base')
    case(f, 'edit({i})', 'One', call(w, [fn('One', lref(I)), fn('Two', L)], [braced(lv(I))]))
    case(f, 'look({i})', 'One', call(w, [fn('One', lref(I)), fn('Two', lref(const(I)))], [braced(lv(I))]))
    case(f, 'take({i})', 'Two', call(w, [fn('One', rref(I)), fn('Two', L)], [braced(lv(I))]))
    case(f, 'keep({i})', 'Two', call(w, [fn('One', rref(const(I))), fn('Two', lref(const(I)))], [braced(lv(I))]))
    case(f, 'part({d})', 'One', call(w, [fn('One', lref(Bs)), fn('Two', lref(const(Bs)))], [braced(lv(cls('Derived')))]))
    case(f, 'whole({pair})', 'One', call(w, [fn('One', lref(arr(I, 2))), fn('Two', L)], [braced(lv(arr(I, 2)))]))
    f = 'list-reference-temporary.cpp'
    w = World(); w.add_class('Point', aggregate=True, fields=[('x', I), ('y', I)]); w.add_class('Single', aggregate=True, fields=[('x', I)])
    case(f, 'take({s})', 'One', call(w, [fn('One', rref(I)), fn('Two', L)], [braced(lv(S))]))
    case(f, 'move({1})', 'One', call(w, [fn('One', rref(I)), fn('Two', lref(const(I)))], [braced(pr(I))]))
    case(f, 'put({.x=1}, 0)', 'One', call(w, [fn('One', lref(const(cls('Point'))), I), fn('Two', cls('Single'), L)],
                                          [braced(pr(I), designators=['x']), pr(I)]))

    f = 'list-scalar.cpp'
    w = World(); w.add_class('Box', ctors=[fn('Box(int)', I)]); w.add_class('Empty', ctors=[fn('Empty()')])
    case(f, 'take({})', 'One', call(w, [fn('One', I), fn('Two', cls('Empty'))], [braced()]))
    case(f, 'one({1})', 'One', call(w, [fn('One', I), fn('Two', L)], [braced(pr(I))]))
    case(f, 'put({{1}})', 'Two', call(w, [fn('One', I), fn('Two', cls('Box'))], [braced(braced(pr(I)))]))
    f = 'list-scalar-pointer-and-pair.cpp'
    w = World(); w.add_class('Pair', ctors=[fn('Pair(int,int)', I, I)])
    case(f, 'take({})', 'One', call(w, [fn('One', ptr(I)), fn('Two', cls('Pair'))], [braced()]))
    case(f, 'real({1.5f})', 'One', call(w, [fn('One', D), fn('Two', L)], [braced(pr(F))]))
    case(f, 'two({1,2})', 'Two', call(w, [fn('One', I), fn('Two', cls('Pair'))], [braced(pr(I), pr(I))]))

    f = 'list-string-literal-binds-literal.cpp'
    w = World()
    case(f, 'known({"ab"})', 'One', call(w, [fn('One', lref(arr(const(C), 3))), fn('Two', rref(arr(const(C), 3)))], [braced(lit(2))]))
    case(f, 'unknown({"ab"})', 'One', call(w, [fn('One', lref(arr(const(C)))), fn('Two', rref(arr(const(C))))], [braced(lit(2))]))
    f = 'list-string-literal-bound.cpp'
    case(f, 'take({"ab"})', 'Two', call(w, [fn('One', lref(arr(const(C)))), fn('Two', lref(arr(const(C), 3)))], [braced(lit(2))]))
    f = 'list-string-literal.cpp'
    w = World(); w.add_class('Wide', ctors=[fn('Wide(const wchar_t*)', ptr(const(fund('wchar_t'))))])
    w.add_class('Text', ctors=[fn('Text(const char*)', ptr(const(C)))])
    case(f, 'narrow({L"ab"})', 'Two', call(w, [fn('One', lref(arr(const(C), 3))), fn('Two', cls('Wide'))], [braced(lit(2, 'wchar_t'))]))
    case(f, 'plain({"ab"})', 'One', call(w, [fn('One', lref(arr(const(C), 3))), fn('Two', cls('Text'))], [braced(lit(2))]))
    case(f, 'kind({"ab"})', 'One', call(w, [fn('One', lref(arr(const(C), 3))), fn('Two', lref(arr(const(fund('wchar_t')), 3)))], [braced(lit(2))]))
    f = 'list-string-literal-too-short.cpp'
    case(f, 'take({"ab"})', 'Two', call(w, [fn('One', lref(arr(const(C), 2))), fn('Two', ptr(const(C)))], [braced(lit(2))]))
    f = 'list-string-literal-utf8.cpp'
    w = World(); w.add_class('Utf8', ctors=[fn('Utf8(const char8_t*)', ptr(const(fund('char8_t'))))])
    case(f, 'take({u8"ab"})', 'One', call(w, [fn('One', lref(arr(const(C), 3))), fn('Two', cls('Utf8'))], [braced(lit(2, 'char8_t'))]))

    f = 'nearest-base.cpp'
    w = World(); chain(w)
    Bs, M, Dv = cls('Base'), cls('Middle'), cls('Derived')
    case(f, 'ptr(dp)', 'Two', call(w, [fn('One', ptr(Bs)), fn('Two', ptr(M))], [lv(ptr(Dv))]))
    case(f, 'ref(d)', 'Two', call(w, [fn('One', lref(Bs)), fn('Two', lref(M))], [lv(Dv)]))
    case(f, 'val(d)', 'Two', call(w, [fn('One', Bs), fn('Two', M)], [lv(Dv)]))
    case(f, 'mem(&Base::x)', 'One', call(w, [fn('One', mptr('Middle', I)), fn('Two', mptr('Derived', I))], [pr(mptr('Base', I))]))

    f = 'object-conversion-function-class.cpp'
    w = World(); w.add_class('Base', convs=[fn('op int', ret=I, cv=k)])
    w.add_class('Derived', bases=['Base'], convs=[fn('op long', ret=L, cv=k)])
    case(f, 'int chosen = Derived{}', 'op int', conv_init(w, pr(cls('Derived')), I))

    f = 'object-explicit-assignment.cpp'
    w = World(); w.add_class('Handle', ctors=[fn('Handle(Target&)', lref(cls('Target')))])
    w.add_class('Target')
    Tg = cls('Target')
    ms = [fn('One', cls('Handle'), I, kind='explicit', owner='Target'),
          fn('copy=', lref(const(Tg)), kind='member', owner='Target'), fn('move=', rref(Tg), kind='member', owner='Target')]
    case(f, 't = 1', 'One', call(w, ms, [lv(Tg), pr(I)], has_object=True))
    case(f, 't += 1', 'Two', call(w, [fn('Two', cls('Handle'), I, kind='explicit', owner='Target')], [lv(Tg), pr(I)], has_object=True))
    f = 'object-explicit-user-defined.cpp'
    w = World(); w.add_class('Item', convs=[fn('op int', ret=I, cv=k)])
    case(f, 'Item{}.get()', 'One', call(w, [fn('One', I, kind='explicit', owner='Item')], [pr(cls('Item'))], has_object=True))
    f = 'object-ref-qualifier.cpp'
    w = World(); w.add_class('Item'); w.add_class('Fixed')
    cs = [fn('One', kind='member', owner='Item', refq='&'), fn('Two', kind='member', owner='Item', refq='&&')]
    case(f, 'i.get()', 'One', call(w, cs, [lv(cls('Item'))], has_object=True))
    case(f, 'Item{}.get()', 'Two', call(w, cs, [pr(cls('Item'))], has_object=True))
    cs = [fn('One', kind='member', owner='Fixed', refq='&', cv=k), fn('Two', kind='member', owner='Fixed', refq='&&')]
    case(f, 'Fixed{}.get()', 'Two', call(w, cs, [pr(cls('Fixed'))], has_object=True))
    f = 'object-rvalue-non-const.cpp'
    w = World(); w.add_class('Item')
    cs = [fn('One', kind='member', owner='Item'), fn('Two', kind='member', owner='Item', cv=k)]
    case(f, 'Item{}.get()', 'One', call(w, cs, [pr(cls('Item'))], has_object=True))
    f = 'object-using-declaration.cpp'
    w = World(); w.add_class('Base'); w.add_class('Derived', bases=['Base'])
    cs = [fn('One', L, I, kind='member', owner='Derived'), fn('Two', U, L, kind='member', owner='Derived')]
    case(f, 'd.get(1, 0)', 'One', call(w, cs, [lv(cls('Derived')), pr(I), pr(I)], has_object=True))
    cs = [fn('One', L, L, kind='member', owner='Derived'), fn('Two', U, I, kind='member', owner='Derived')]
    case(f, 'd2.get(1, 0)', 'Two', call(w, cs, [lv(cls('Derived')), pr(I), pr(I)], has_object=True))

    f = 'overload-set-argument-sequence.cpp'
    w = World()
    vi, vd, vc = fnty(VOID, [I]), fnty(VOID, [D]), fnty(VOID, [C])
    twice = Arg(overload_set=[(vi, False), (vd, False)])
    once = Arg(overload_set=[(vi, False)])
    exact = Arg(overload_set=[(fnty(VOID, [I], True), False), (vd, False)])
    generic = Arg(overload_set=[(vi, False)])
    case(f, 'a::g(twice)', 'Two', call(w, [fn('One', B), fn('Two', ptr(vi))], [twice]))
    case(f, 'a::g(once)', 'Two', call(w, [fn('One', B), fn('Two', ptr(vi))], [once]))
    case(f, 'b::g(once)', 'One', call(w, [fn('One', B), fn('Two', ellipsis=True)], [once]))
    case(f, 'c::g(once)', 'Two', call(w, [fn('One', ptr(vc)), fn('Two', ellipsis=True)], [once]))
    case(f, 'c::g(generic<int>)', 'Two', call(w, [fn('One', ptr(vc)), fn('Two', ellipsis=True)], [generic]))
    case(f, 'd::g(exact)', 'Two', call(w, [fn('One', ptr(vi)), fn('Two', ptr(fnty(VOID, [I], True)))], [exact]))
    case(f, 'e::g(twice)', 'One', call(w, [fn('One', ptr(vi)), fn('Two', ellipsis=True)], [twice]))
    f = 'overload-set-reference-or-pointer.cpp'
    case(f, 'g(f)', 'ambiguous', call(w, [fn('ref', lref(vi)), fn('ptr', ptr(vi))], [twice]))
    f = 'overload-set-to-ellipsis.cpp'
    case(f, 'g(f)', 'no viable function', call(w, [fn('g', ellipsis=True)], [twice]))

    f = 'pointer-to-bool.cpp'
    w = World(); w.add_class('Item')
    case(f, 'take(p)', 'Two', call(w, [fn('One', B), fn('Two', ptr(VOID))], [lv(ptr(I))]))
    case(f, 'member(&Item::x)', 'One', call(w, [fn('One', B), fn('Two', ellipsis=True)], [pr(mptr('Item', I))]))
    f = 'qualification-array-bound.cpp'
    case(f, 'take(row)', 'One', call(w, [fn('One', ptr(arr(const(I), 3))), fn('Two', ptr(arr(const(I))))], [lv(ptr(arr(I, 3)))]))
    f = 'qualification-fewer.cpp'
    case(f, 'take(p)', 'One', call(w, [fn('One', ptr(const(I))), fn('Two', ptr(cv(I)))], [lv(ptr(I))]))
    case(f, 'deep(pp)', 'One', call(w, [fn('One', ptr(const(ptr(const(I))))), fn('Two', ptr(cv(ptr(cv(I)))))], [lv(ptr(ptr(I)))]))
    f = 'qualification-reference-temporary.cpp'
    case(f, 'take(&n)', 'One', call(w, [fn('One', lref(const(ptr(const(I))))), fn('Two', ptr(cv(I)))], [pr(ptr(I))]))
    f = 'rank-before-reference.cpp'
    case(f, 'take(1)', 'Two', call(w, [fn('One', rref(L)), fn('Two', lref(const(I)))], [pr(I)]))


import score_tests
score_tests.BATCHES.append(batch2)
