from score_tests import *  # noqa


def batch6():
    f = 'declarations/functions/definitions/different-parameter-types-overload.cpp'
    w = World(); w.add_class('Gauge'); G = cls('Gauge')
    ks = [fn('1', I), fn('2', L), fn('3', ptr(I))]
    case(f, 'kind(0)', '1', call(w, ks, [Arg(I, 'prvalue', null=True)]))
    case(f, 'kind(0L)', '2', call(w, ks, [Arg(L, 'prvalue', null=True)]))
    case(f, 'kind(&v)', '3', call(w, ks, [pr(ptr(I))]))
    rs = [fn('4', kind='member', owner='Gauge', refq='&'), fn('5', kind='member', owner='Gauge', refq='&&')]
    case(f, 'g.read()', '4', call(w, rs, [lv(G)], has_object=True))
    case(f, 'Gauge{}.read()', '5', call(w, rs, [pr(G)], has_object=True))
    ps = [fn('6', kind='member', owner='Gauge'), fn('7', kind='member', owner='Gauge', cv=k)]
    case(f, 'g.peek()', '6', call(w, ps, [lv(G)], has_object=True))
    case(f, 'cg.peek()', '7', call(w, ps, [lv(const(G))], has_object=True))
    f = 'foundations/language-support/initializer-list-candidate-not-viable.cpp'
    w = World(); w.add_class('Point', aggregate=True, fields=[('x', I), ('y', I)])
    case(f, 'take({1, 2})', 'Point', call(w, [fn('il', ilist(ptr(I))), fn('Point', cls('Point'))], [braced(pr(I), pr(I))]))


import score_tests
score_tests.BATCHES.append(batch6)
