"""Score the blind layout.py on the layout assertions of the tests.

The class descriptions are written by hand from each test file. Each
comparison inside a static_assert (each side of an `&&`) is one check.
Run-time offset checks in `run` tests are scored apart, in RUN.
Compiler variants (GCC 14, EDG) are scored apart, in VARIANT: the test
expects the assertion to fail on that compiler.
"""
import sys
sys.path.insert(0, __file__.rsplit('/', 1)[0])
from layout import Cls, Base, Mem, layout

L64, L32 = 'LP64', 'ILP32'
CHECKS = []   # (file, label, got_fn, expected)
RUN = []
VARIANT = []


def sz(c, t=L64, comp='gcc16'):
    return layout(c, t, comp).size


def al(c, t=L64, comp='gcc16'):
    return layout(c, t, comp).align


def chk(file, label, fn, exp, into=None):
    (CHECKS if into is None else into).append((file, label, fn, exp))


def S(name, *members, bases=(), **kw):
    bl = [b if isinstance(b, Base) else Base(b) for b in bases]
    return Cls(name, bases=bl, members=list(members), **kw)


def V(c):
    return Base(c, True)


def M(name, t, **kw):
    return Mem(name, t, **kw)


def BF(name, t, w):
    return Mem(name, t, width=w)


# ---------------------------------------------------------------- CLS-11
f = 'classes/members/empty-class-array-size.cpp'
E = S('Empty')
chk(f, 'sizeof(Empty)==1', lambda: sz(E), 1)
chk(f, 'sizeof(Empty[4])==4', lambda: 4 * sz(E), 4)
f = 'classes/members/empty-class-size-one.cpp'
chk(f, 'sizeof(Empty)==1', lambda: sz(E), 1)
f = 'classes/members/empty-class-size-one-ilp32.cpp'
chk(f, 'sizeof(Empty)==1', lambda: sz(E, L32), 1)
chk(f, 'sizeof(Empty[4])==4', lambda: 4 * sz(E, L32), 4)

# ---------------------------------------------------------------- CLS-33
for f, t in (('classes/members/empty-standard-layout-base-zero-size.cpp', L64),
             ('classes/members/empty-standard-layout-base-zero-size-ilp32.cpp', L32)):
    E = S('Empty')
    Shape = S('Shape', M('id', 'int'), bases=[E], virtual_funcs=True)
    PlainShape = S('PlainShape', M('id', 'int'), virtual_funcs=True)
    Account = S('Account', M('number', 'int'), M('balance', 'int', access='private', init=True), bases=[E])
    PlainAccount = S('PlainAccount', M('number', 'int'), M('balance', 'int', access='private', init=True))
    chk(f, 'sizeof(Shape)==sizeof(PlainShape)', lambda Shape=Shape, t=t: sz(Shape, t), sz(PlainShape, t))
    chk(f, 'sizeof(Account)==sizeof(PlainAccount)', lambda Account=Account, t=t: sz(Account, t), sz(PlainAccount, t))


# ---------------------------------------------------------------- tail padding
def ac(name, **kw):
    """struct name { int a; char b; } with flags."""
    acc_b = kw.pop('acc_b', 'public')
    init_a = kw.pop('init_a', False)
    return S(name, M('a', 'int', init=init_a), M('b', 'char', access=acc_b), **kw)


def Next(B, nm=None):
    return S('Next<%s>' % B.name, M('c', 'char'), bases=[B])


for f, t in (('classes/members/layout-base-tail-padding.cpp', L64),
             ('classes/members/layout-base-tail-padding-ilp32.cpp', L32)):
    Plain = ac('Plain')
    Init = ac('Initialized', init_a=True)
    Built = ac('Built', user_ctor=True)
    Assigned = ac('Assigned', user_copy_assign=True)
    Ended = ac('Ended', user_dtor=True)
    Hidden = ac('Hidden', acc_b='private')
    Guarded = ac('Guarded', acc_b='protected')
    Derived = S('Derived', bases=[Plain])
    for B, e in ((Plain, 12), (Derived, 12), (Init, 8), (Built, 8), (Assigned, 8),
                 (Ended, 8), (Hidden, 8), (Guarded, 8)):
        chk(f, 'sizeof(Next<%s>)==%d' % (B.name, e), lambda B=B, t=t: sz(Next(B), t), e)

f = 'classes/members/layout-base-tail-padding-defaulted.cpp'
Defaulted = ac('Defaulted', defaulted_ctor=True)
chk(f, 'sizeof(Next)==12', lambda: sz(Next(Defaulted)), 12)
chk(f, 'gcc-14: sizeof(Next)==12 fails', lambda: sz(Next(Defaulted), L64, 'gcc14') != 12, True, VARIANT)

f = 'classes/members/layout-base-type-first-member.cpp'
Unit = S('Unit')
Wrapper = S('Wrapper', M('head', Unit), M('value', 'int'), bases=[Unit])
chk(f, 'sizeof(Wrapper)==8 (LP64)', lambda: sz(Wrapper), 8)

f = 'classes/members/layout-empty-base-not-standard-layout.cpp'
Unit = S('Unit')
Left = S('Left', bases=[Unit])
Right = S('Right', bases=[Unit])
Both = S('Both', bases=[Left, Right])
User = S('User', M('value', 'int'), bases=[Both])
chk(f, 'sizeof(Both)==2', lambda: sz(Both), 2)
chk(f, 'sizeof(User)==sizeof(int)', lambda: sz(User), 4)

f = 'classes/members/layout-empty-bases-depths.cpp'
Tag = S('Tag')
Red = S('Red', bases=[Tag])
Blue = S('Blue', bases=[Tag])
Number = S('Number', M('n', 'int'))
Depths = S('Depths', M('x', 'int'), bases=[Red, Blue])
Nested = S('Nested', bases=[Tag, Red])
AfterData = S('AfterData', bases=[Number, Red, Blue])
chk(f, 'sizeof(Depths)==4', lambda: sz(Depths), 4)
chk(f, 'sizeof(Nested)==2', lambda: sz(Nested), 2)
chk(f, 'sizeof(AfterData)==8', lambda: sz(AfterData), 8)
chk(f, 'Depths: Red 0', lambda: layout(Depths).bases['Red'], 0, RUN)
chk(f, 'Depths: Blue 1', lambda: layout(Depths).bases['Blue'], 1, RUN)
chk(f, 'Depths: x 0', lambda: layout(Depths).offsets['x'], 0, RUN)
chk(f, 'Nested: Red 1', lambda: layout(Nested).bases['Red'], 1, RUN)
chk(f, 'AfterData: Red 0', lambda: layout(AfterData).bases['Red'], 0, RUN)
chk(f, 'AfterData: Blue 4', lambda: layout(AfterData).bases['Blue'], 4, RUN)

f = 'classes/members/layout-empty-member-after-empty-base.cpp'
Tag = S('Tag')
HoldsTag = S('HoldsTag', M('t', Tag))
MemberAfter = S('MemberAfter', M('e', Tag), M('x', 'int'), bases=[Tag])
HoldsAfter = S('HoldsAfter', M('m', HoldsTag), bases=[Tag])
AlignedAfter = S('AlignedAfter', M('e', Tag, alignas=8), M('x', 'int'), bases=[Tag])
chk(f, 'sizeof(MemberAfter)==8', lambda: sz(MemberAfter), 8)
chk(f, 'sizeof(HoldsAfter)==2', lambda: sz(HoldsAfter), 2)
chk(f, 'sizeof(AlignedAfter)==16', lambda: sz(AlignedAfter), 16)
chk(f, 'MemberAfter: e 1', lambda: layout(MemberAfter).offsets['e'], 1, RUN)
chk(f, 'MemberAfter: x 4', lambda: layout(MemberAfter).offsets['x'], 4, RUN)
chk(f, 'HoldsAfter: m 1', lambda: layout(HoldsAfter).offsets['m'], 1, RUN)
chk(f, 'AlignedAfter: e 8', lambda: layout(AlignedAfter).offsets['e'], 8, RUN)
chk(f, 'AlignedAfter: x 12', lambda: layout(AlignedAfter).offsets['x'], 12, RUN)

# ---------------------------------------------------------------- order and pointer
for f, t, exp in (('classes/members/layout-order-and-pointer.cpp', L64,
                   dict(Listed=12, OwnPointer=16, UsesBase=16, Both=24, Middle=16, Last=16, Padded=24, aPadded=8)),
                  ('classes/members/layout-order-and-pointer-ilp32.cpp', L32,
                   dict(Listed=12, OwnPointer=12, UsesBase=12, Both=12, Middle=12, Last=12, Padded=16, aPadded=4))):
    Number = S('Number', M('x', 'int'))
    Letter = S('Letter', M('y', 'char'))
    Dynamic = S('Dynamic', virtual_funcs=True)
    Other = S('Other', virtual_funcs=True)
    Shared = S('Shared', M('v', 'int'))
    cl = dict(
        Listed=S('Listed', M('z', 'char'), bases=[Letter, Number]),
        OwnPointer=S('OwnPointer', M('y', 'int'), bases=[Number], virtual_funcs=True),
        UsesBase=S('UsesBase', M('y', 'int'), bases=[Number, Dynamic], virtual_funcs=True),
        Both=S('Both', M('y', 'int'), bases=[Dynamic, Other]),
    )
    cl['Middle'] = S('Middle', M('y', 'char'), bases=[V(Shared)])
    cl['Last'] = S('Last', M('z', 'char'), bases=[cl['Middle']])
    cl['Padded'] = S('Padded', M('a', 'char'), M('d', 'double'), M('e', 'char'))
    for k, e in exp.items():
        if k == 'aPadded':
            chk(f, 'alignof(Padded)==%d' % e, lambda c=cl['Padded'], t=t: al(c, t), e)
        else:
            chk(f, 'sizeof(%s)==%d' % (k, e), lambda c=cl[k], t=t: sz(c, t), e)

# ---------------------------------------------------------------- no_unique_address
for f, t in (('classes/members/layout-overlap-member.cpp', L64),
             ('classes/members/layout-overlap-member-ilp32.cpp', L32)):
    E = S('Empty')
    Plain = ac('Plain')
    Built = ac('Built', user_ctor=True)
    Beside = S('Beside', M('value', 'int'), M('tag', E, nua=True))
    Two = S('Two', M('first', E, nua=True), M('second', E, nua=True), M('value', 'int'))
    Only = S('Only', M('first', E, nua=True), M('second', E, nua=True))
    PlainThen = S('PlainThen', M('part', Plain, nua=True), M('c', 'char'))
    BuiltThen = S('BuiltThen', M('part', Built, nua=True), M('c', 'char'))
    chk(f, 'sizeof(Beside)==sizeof(int)', lambda c=Beside, t=t: sz(c, t), 4)
    chk(f, 'sizeof(Two)==sizeof(int)', lambda c=Two, t=t: sz(c, t), 4)
    chk(f, 'sizeof(Only)==2', lambda c=Only, t=t: sz(c, t), 2)
    chk(f, 'sizeof(PlainThen)==12', lambda c=PlainThen, t=t: sz(c, t), 12)
    chk(f, 'sizeof(BuiltThen)==8', lambda c=BuiltThen, t=t: sz(c, t), 8)
    if t == L64:
        chk(f, 'Two: first != second, both < 4', lambda c=Two: (lambda o: o['first'] != o['second'] and o['first'] < 4 and o['second'] < 4)(layout(c).offsets), True, RUN)

# ---------------------------------------------------------------- tail padding conditions
for f, t in (('classes/members/layout-tail-padding-conditions.cpp', L64),
             ('classes/members/layout-tail-padding-conditions-ilp32.cpp', L32)):
    HasCtor = S('HasConstructor', M('x', 'char'), user_ctor=True)
    HasMove = S('HasMoveAssignment', M('x', 'char'))
    PlainSmall = S('PlainSmall', M('x', 'char'))
    E = S('Empty')

    def lends(B, t=t):
        return sz(Next(B), t) == sz(B, t)

    def keeps(B, t=t):
        return sz(Next(B), t) > sz(B, t)
    L = [
        S('WithBase', M('a', 'int'), M('b', 'char'), bases=[E]),
        S('WithVirtual', M('a', 'int'), M('b', 'char'), virtual_funcs=True),
        S('WithReference', M('r', 'ref'), M('a', 'int'), M('b', 'char')),
        ac('Converting', user_ctor=True), ac('Explicit', user_ctor=True),
        ac('Template', ctor_template=True),
        ac('Copying', user_ctor=True, defaulted_ctor=True),
        ac('Moving', user_ctor=True, defaulted_ctor=True),
        ac('OutOfLine', user_ctor=True),
        ac('ByValue', user_copy_assign=True),
        S('MemberLends', M('m', HasCtor), M('a', 'int'), M('b', 'char')),
        S('ArrayLends', M('m', HasCtor, bounds=(1,)), M('a', 'int'), M('b', 'char')),
    ]
    K = [
        S('MemberKeeps', M('m', PlainSmall), M('a', 'int'), M('b', 'char')),
        S('MemberMoveDefaulted', M('m', HasMove), M('a', 'int'), M('b', 'char')),
        S('ConstMember', M('a', 'int'), M('b', 'char')),
        S('PointerMember', M('p', 'ptr'), M('a', 'int'), M('b', 'char')),
        ac('PrivateFunction'), ac('PrivateStatic'), ac('DeletedDestructor'),
        ac('DefaultedAssignment'),
    ]
    for B in L:
        chk(f, 'lends<%s>' % B.name, lambda B=B: lends(B), True)
    for B in K:
        chk(f, 'keeps<%s>' % B.name, lambda B=B: keeps(B), True)
    Built = ac('Built', user_ctor=True)
    Middle = S('Middle', M('d', 'char'), bases=[Built])
    Ordinary = S('Ordinary', M('part', Built), M('c', 'char'))
    chk(f, 'sizeof(Middle)==8', lambda Middle=Middle, t=t: sz(Middle, t), 8)
    chk(f, 'sizeof(Next<Middle>)==8', lambda Middle=Middle, t=t: sz(Next(Middle), t), 8)
    chk(f, 'sizeof(Ordinary)==12', lambda Ordinary=Ordinary, t=t: sz(Ordinary, t), 12)

for f, t in (('classes/members/layout-tail-padding-deleted-constructor.cpp', L64),
             ('classes/members/layout-tail-padding-deleted-constructor-ilp32.cpp', L32)):
    NoDefault = ac('NoDefault', deleted_ctor=True)
    NoCopy = ac('NoCopy', deleted_ctor=True, defaulted_ctor=True)
    NoMove = ac('NoMove', deleted_ctor=True, defaulted_ctor=True)
    for B in (NoDefault, NoCopy, NoMove):
        chk(f, 'sizeof(Next<%s>)==12' % B.name, lambda B=B, t=t: sz(Next(B), t), 12)
        for comp in ('gcc14', 'edg'):
            chk(f, '%s: sizeof(Next<%s>)==12 fails' % (comp, B.name),
                lambda B=B, t=t, comp=comp: sz(Next(B), t, comp) != 12, True, VARIANT)

for f, t in (('classes/members/layout-tail-padding-move-assignment.cpp', L64),
             ('classes/members/layout-tail-padding-move-assignment-ilp32.cpp', L32)):
    Moved = ac('Moved', user_move_assign=True)
    Holder = S('Holder', M('part', Moved, nua=True), M('c', 'char'))
    chk(f, 'sizeof(Next)==12', lambda Moved=Moved, t=t: sz(Next(Moved), t), 12)
    chk(f, 'sizeof(Holder)==12', lambda Holder=Holder, t=t: sz(Holder, t), 12)
    chk(f, 'edg: sizeof(Next)==12 fails', lambda Moved=Moved, t=t: sz(Next(Moved), t, 'edg') != 12, True, VARIANT)
    chk(f, 'edg: sizeof(Holder)==12 fails', lambda Holder=Holder, t=t: sz(Holder, t, 'edg') != 12, True, VARIANT)

f = 'classes/members/layout-tail-padding-assignment-forms.cpp'
for nm in ('ByReference', 'ByConst', 'ByVolatile', 'ByConstVolatile', 'ByValue', 'ByConstValue'):
    B = ac(nm, user_copy_assign=True)
    chk(f, 'lends<%s>' % nm, lambda B=B: sz(Next(B)) == sz(B), True)
for nm in ('AssignTemplate', 'AssignOther'):
    B = ac(nm)
    chk(f, 'keeps<%s>' % nm, lambda B=B: sz(Next(B)) > sz(B), True)
B = ac('PackConstructor', ctor_template=True)
chk(f, 'lends<PackConstructor>', lambda B=B: sz(Next(B)) == sz(B), True)

# ---------------------------------------------------------------- virtual bases
for f, t, e in (('classes/members/layout-virtual-base-part.cpp', L64, (16, 16, 24, 24, 40)),
                ('classes/members/layout-virtual-base-part-ilp32.cpp', L32, (12, 12, 20, 16, 24))):
    Shared = S('Shared', M('v', 'int'))
    Number = S('Number', M('n', 'int'))
    Middle = S('Middle', M('y', 'char'), bases=[V(Shared)])
    Last = S('Last', M('z', 'char'), bases=[Middle])
    Second = S('Second', M('z', 'char'), bases=[Number, Middle])
    First = S('First', M('a', 'int'))
    Other = S('Other', M('b', 'int'))
    Two = S('Two', M('c', 'char'), bases=[V(First), V(Other)])
    Left = S('Left', M('l', 'char'), bases=[V(Other)])
    Right = S('Right', M('r', 'char'), bases=[V(First)])
    Graph = S('Graph', M('g', 'char'), bases=[Left, Right])
    for c, x in zip((Middle, Last, Second, Two, Graph), e):
        chk(f, 'sizeof(%s)==%d' % (c.name, x), lambda c=c, t=t: sz(c, t), x)

f = 'classes/members/layout-virtual-base-order.cpp'
First = S('First', M('a', 'int'))
Other = S('Other', M('b', 'int'))
Left = S('Left', M('l', 'char'), bases=[V(Other)])
Right = S('Right', M('r', 'char'), bases=[V(First)])
Graph = S('Graph', M('g', 'char'), bases=[Left, Right])
Deep = S('Deep', bases=[Right, V(Other)])
Direct = S('Direct', bases=[V(Other), Right])
for c, k, x in ((Graph, 'Right', 16), (Graph, 'Other', 28), (Graph, 'First', 32),
                (Deep, 'First', 12), (Deep, 'Other', 16),
                (Direct, 'Other', 12), (Direct, 'First', 16)):
    chk(f, '%s: %s %d' % (c.name, k, x), lambda c=c, k=k: layout(c).bases[k], x, RUN)

f = 'classes/members/layout-virtual-first-member.cpp'
Shape = S('Shape', M('id', 'int', init=True), virtual_funcs=True)
chk(f, 'Shape: id at sizeof(void*)', lambda: layout(Shape).offsets['id'], 8, RUN)
chk(f, 'sizeof(Shape)==2*sizeof(void*)', lambda: sz(Shape), 16, RUN)
f = 'classes/members/layout-virtual-first-member-ilp32.cpp'
chk(f, 'sizeof(Shape)==8', lambda: sz(Shape, L32), 8)
chk(f, 'alignof(Shape)==4', lambda: al(Shape, L32), 4)

f = 'classes/members/standard-layout-empty-bases.cpp'
Tag, Mark = S('Tag'), S('Mark')
Deep = S('Deep', bases=[Tag])
Record = S('Record', M('id', 'int'), bases=[Deep, Mark])
chk(f, 'sizeof(Record)==sizeof(int)', lambda: sz(Record), 4)
chk(f, 'Record: id, Tag, Mark at 0', lambda: (layout(Record).offsets['id'], layout(Record).bases['Deep'], layout(Record).bases['Mark']), (0, 0, 0), RUN)

# ---------------------------------------------------------------- bit-fields
f = 'classes/unions/bitfield-layout-lp64.cpp'
chk(f, 'sizeof(S)==sizeof(unsigned)', lambda: sz(S('S', BF('a', 'unsigned', 3), BF('b', 'unsigned', 5), BF('c', 'unsigned', 8))), 4)
chk(f, 'sizeof(T)==1', lambda: sz(S('T', BF('a', 'char', 4), BF('b', 'char', 4))), 1)

for f, t, e in (('classes/unions/bitfield-next-unit-lp64.cpp', L64, dict(Split=8, Shorts=4, aShorts=2, Mixed=8, aMixed=8, Long=16, Cross=16)),
                ('classes/unions/bitfield-next-unit-ilp32.cpp', L32, dict(Split=8, Shorts=4, aShorts=2, Mixed=8, aMixed=4, Long=12, Cross=12))):
    cl = dict(
        Split=S('Split', BF('a', 'unsigned', 20), BF('b', 'unsigned', 20)),
        Shorts=S('Shorts', BF('a', 'unsigned char', 7), BF('b', 'unsigned short', 12)),
        Mixed=S('Mixed', M('c', 'char'), BF('b', 'unsigned long long', 30)),
        Long=S('Long', BF('a', 'unsigned long long', 40), BF('b', 'unsigned long long', 30)),
        Cross=S('Cross', BF('a', 'unsigned', 30), BF('b', 'unsigned long long', 40)),
    )
    for k, x in e.items():
        if k.startswith('a'):
            chk(f, 'alignof(%s)==%d' % (k[1:], x), lambda c=cl[k[1:]], t=t: al(c, t), x)
        else:
            chk(f, 'sizeof(%s)==%d' % (k, x), lambda c=cl[k], t=t: sz(c, t), x)
    if t == L32:
        for k in ('Mixed', 'Long', 'Cross'):
            ex = {'Mixed': (lambda c: (sz(c, L32, 'edg'), al(c, L32, 'edg')) != (8, 4)),
                  'Long': (lambda c: sz(c, L32, 'edg') != 12),
                  'Cross': (lambda c: sz(c, L32, 'edg') != 12)}[k]
            chk(f, 'edg: %s assertion fails' % k, lambda c=cl[k], ex=ex: ex(c), True, VARIANT)

f = 'classes/unions/bitfield-union-size.cpp'
for t in (L64, L32):
    for c, s, a in ((S('Nibble', BF('x', 'unsigned', 4), union=True), 4, 4),
                    (S('Twelve', BF('c', 'unsigned char', 12), union=True), 2, 1),
                    (S('Forty', BF('x', 'int', 40), union=True), 8, 4),
                    (S('Small', BF('x', 'short', 3), M('y', 'char', bounds=(3,)), union=True), 4, 2)):
        chk(f, '%s: sizeof(%s)==%d' % (t, c.name, s), lambda c=c, t=t: sz(c, t), s)
        chk(f, '%s: alignof(%s)==%d' % (t, c.name, a), lambda c=c, t=t: al(c, t), a)

for f, t in (('classes/unions/bitfield-unnamed-alignment.cpp', L64),
             ('classes/unions/bitfield-unnamed-alignment-ilp32.cpp', L32)):
    for c, a, s in ((S('WideGap', M('a', 'char'), BF(None, 'long long', 3)), 1, 2),
                    (S('IntGap', M('a', 'char'), BF(None, 'int', 0), M('b', 'char')), 1, 5),
                    (S('EndGap', M('a', 'char'), BF(None, 'int', 0)), 1, 4)):
        chk(f, 'alignof(%s)==%d' % (c.name, a), lambda c=c, t=t: al(c, t), a)
        chk(f, 'sizeof(%s)==%d' % (c.name, s), lambda c=c, t=t: sz(c, t), s)

f = 'classes/unions/bitfield-wide-alignment-128.cpp'
Wide = S('Wide', BF('x', 'int', 128))
Huge = S('Huge', BF('x', 'int', 1000))
chk(f, 'sizeof(Wide)==16 (Clang, EDG)', lambda: sz(Wide), 16)
chk(f, 'alignof(Wide)==8 (Clang, EDG)', lambda: al(Wide), 8)
chk(f, 'sizeof(Huge)==128 (Clang, EDG)', lambda: sz(Huge), 128)
chk(f, 'alignof(Huge)==8 (Clang, EDG)', lambda: al(Huge), 8)

for f, t, e in (('classes/unions/bitfield-wide-layout.cpp', L64,
                 [('Forty', 8, 4), ('SixtyFour', 8, 8), ('AfterChar', 24, 8), ('AfterBits', 12, None), ('Shared', 3, 1), ('Sixteen', 2, 2)]),
                ('classes/unions/bitfield-wide-layout-ilp32.cpp', L32,
                 [('Forty', 8, 4), ('SixtyFour', 8, 4), ('AfterChar', 16, 4), ('AfterBits', 12, None), ('Shared', 3, 1), ('Sixteen', 2, 2)])):
    cl = dict(
        Forty=S('Forty', BF('x', 'int', 40), M('d', 'char')),
        SixtyFour=S('SixtyFour', BF('x', 'int', 64)),
        AfterChar=S('AfterChar', M('a', 'char'), BF('x', 'int', 65)),
        AfterBits=S('AfterBits', BF('a', 'char', 4), BF('x', 'int', 40)),
        Shared=S('Shared', BF('c', 'unsigned char', 12), BF('e', 'unsigned char', 4), M('d', 'char')),
        Sixteen=S('Sixteen', BF('c', 'char', 16)),
    )
    for k, s, a in e:
        chk(f, 'sizeof(%s)==%d' % (k, s), lambda c=cl[k], t=t: sz(c, t), s)
        if a is not None:
            chk(f, 'alignof(%s)==%d' % (k, a), lambda c=cl[k], t=t: al(c, t), a)
    if t == L32:
        chk(f, 'edg: SixtyFour or AfterChar fails', lambda cl=cl: (al(cl['SixtyFour'], L32, 'edg'), sz(cl['AfterChar'], L32, 'edg'), al(cl['AfterChar'], L32, 'edg')) != (4, 16, 4), True, VARIANT)

f = 'classes/unions/bitfield-zero-width-splits.cpp'
chk(f, 'sizeof(S)==2*sizeof(unsigned)', lambda: sz(S('S', BF('a', 'unsigned', 3), BF(None, 'unsigned', 0), BF('b', 'unsigned', 3))), 8)

f = 'classes/unions/bitfield-low-order-first.cpp'
H = S('Header', BF('low', 'unsigned', 4), BF('high', 'unsigned', 4), BF('rest', 'unsigned', 24))
chk(f, 'low bits 0-3, high bits 4-7', lambda: (layout(H).bitoffsets['low'], layout(H).bitoffsets['high']), (0, 4), RUN)

# ---------------------------------------------------------------- alignment
f = 'declarations/alignment/array-of-aligned.cpp'
chk(f, 'sizeof(S[3])==48', lambda: 3 * sz(S('S', M('v', 'int'), alignas=16)), 48)
f = 'declarations/alignment/base-raises-class.cpp'
Bse = S('Base', M('c', 'char', alignas=16))
Der = S('Derived', M('x', 'char'), bases=[Bse])
Tagged = S('Tagged', alignas=32)
Holder = S('Holder', M('x', 'char'), bases=[Tagged])
chk(f, 'alignof(Derived)==16', lambda: al(Der), 16)
chk(f, 'sizeof(Derived)==32', lambda: sz(Der), 32)
chk(f, 'alignof(Holder)==32', lambda: al(Holder), 32)
f = 'declarations/alignment/class.cpp'
Line = S('Line', M('data', 'char', bounds=(4,)), alignas=32)
chk(f, 'alignof(Line)==32', lambda: al(Line), 32)
chk(f, 'sizeof(Line)==32', lambda: sz(Line), 32)
f = 'declarations/alignment/class-forward-alignas.cpp'
chk(f, 'alignof(S)==16', lambda: al(S('S', M('v', 'int'), alignas=16)), 16)
f = 'declarations/alignment/member.cpp'
SS = S('S', M('c', 'char'), M('v', 'int', alignas=8))
chk(f, 'alignof(S)==8', lambda: al(SS), 8)
chk(f, 'sizeof(S)==16', lambda: sz(SS), 16)
f = 'declarations/alignment/member-raises-class.cpp'
Inner = S('Inner', M('c', 'char', alignas=64))
Outer = S('Outer', M('x', 'char'), M('in', Inner))
chk(f, 'alignof(Outer)==64', lambda: al(Outer), 64)
chk(f, 'sizeof(Outer)==128', lambda: sz(Outer), 128)
f = 'declarations/alignment/strictest-wins.cpp'
chk(f, 'alignof(S)==16 (Clang, EDG; strictest of 4, 16, 8)', lambda: al(S('S', M('c', 'char'), alignas=max(4, 16, 8))), 16)
f = 'declarations/alignment/type-operand.cpp'
chk(f, 'alignof(S)==alignof(double)', lambda: al(S('S', M('c', 'char'), alignas=8)), 8)
f = 'declarations/alignment/union-alignas.cpp'
Small = S('Small', M('c', 'char'), union=True, alignas=8)
Later = S('Later', M('c', 'char'), union=True, alignas=16)
chk(f, 'alignof(Small)==8', lambda: al(Small), 8)
chk(f, 'sizeof(Small)==8', lambda: sz(Small), 8)
chk(f, 'alignof(Later)==16', lambda: al(Later), 16)
f = 'declarations/alignment/zero-ignored.cpp'
chk(f, 'alignof(S)==alignof(int)', lambda: al(S('S', M('v', 'int'), alignas=None)), 4)

f = 'declarations/attributes/no-unique-address.cpp'
E = S('Empty')
chk(f, 'sizeof(Plain)>sizeof(int)', lambda: sz(S('Plain', M('e', E), M('v', 'int'))) > 4, True)
chk(f, 'sizeof(Packed)==sizeof(int)', lambda: sz(S('Packed', M('e', E, nua=True), M('v', 'int'))), 4)

f = 'foundations/storage/empty-base-takes-no-bytes.cpp'
Tag = S('Tag')
Value = S('Value', M('number', 'int'), bases=[Tag])
chk(f, 'sizeof(Value)==sizeof(int)', lambda: sz(Value), 4)
chk(f, 'Tag base at the address of number', lambda: layout(Value).bases['Tag'] == layout(Value).offsets['number'], True, RUN)

f = 'foundations/types/align-long-double-member-ilp32.cpp'
Wd = S('Wide', M('tag', 'char'), M('value', 'long double'))
chk(f, 'sizeof(Wide)==16', lambda: sz(Wd, L32), 16)
chk(f, 'alignof(Wide)==4', lambda: al(Wd, L32), 4)
f = 'foundations/types/align-member-ilp32.cpp'
Pair = S('Pair', M('tag', 'char'), M('value', 'long long'))
Real = S('Real', M('tag', 'char'), M('value', 'double'))
chk(f, 'sizeof(Pair)==12', lambda: sz(Pair, L32), 12)
chk(f, 'sizeof(Real)==12', lambda: sz(Real, L32), 12)
chk(f, 'edg: sizeof(Pair)==12 fails', lambda: sz(Pair, L32, 'edg') != 12, True, VARIANT)

# ---------------------------------------------------------------- programs (x86-64)
f = 'programs/W13/header.cpp'
Kind = ('enum', 'unsigned')
Hdr = S('Header', BF('version', 'unsigned', 4), BF('length_words', 'unsigned', 4),
        BF('service', 'unsigned', 8), BF('total_length', 'unsigned', 16),
        BF(None, 'unsigned', 0), BF('hop_delta', 'int', 5), BF('kind', Kind, 2),
        BF('urgent', 'bool', 1), BF('reserved', 'unsigned', 24))
WideH = S('Wide', BF('id', 'unsigned long long', 40), BF('seq', 'unsigned long long', 24))
chk(f, 'sizeof(Header)==8', lambda: sz(Hdr), 8)
chk(f, 'alignof(Header)==4', lambda: al(Hdr), 4)
chk(f, 'sizeof(Wide)==8', lambda: sz(WideH), 8)
chk(f, 'alignof(Wide)==8', lambda: al(WideH), 8)
f = 'programs/W13/registers.cpp'
Uart = S('UartRegs', *[M(n, 'unsigned') for n in ('status', 'control', 'data', 'count')])
chk(f, 'sizeof(UartRegs)==16', lambda: sz(Uart), 16)
chk(f, 'alignof(UartRegs)==4', lambda: al(Uart), 4)
f = 'programs/W13/aligned.cpp'
Sample = S('Sample', M('time', 'long long'), M('value', 'double'), user_ctor=True, user_dtor=True)
Block = S('Block', M('lanes', 'float', bounds=(4,)), alignas=32)
chk(f, 'alignof(Block)==32', lambda: al(Block), 32)
chk(f, 'sizeof(Block)==32', lambda: sz(Block), 32)
chk(f, 'alignof(Sample)==8', lambda: al(Sample), 8)


def run(lst):
    per = {}
    bad = []
    for file, label, fn, exp in lst:
        try:
            got = fn()
        except Exception as e:  # noqa
            got = 'error: %r' % e
        ok = got == exp
        p = per.setdefault(file, [0, 0])
        p[0] += 1
        p[1] += ok
        if not ok:
            bad.append((file, label, got, exp))
    return per, bad


if __name__ == '__main__':
    for title, lst in (('STATIC', CHECKS), ('RUN', RUN), ('VARIANT', VARIANT)):
        per, bad = run(lst)
        n = sum(v[0] for v in per.values())
        m = sum(v[1] for v in per.values())
        print('== %s: %d checked, %d matched, %d mismatched' % (title, n, m, n - m))
        for file, (a, b) in per.items():
            print('  %-62s %3d %3d %3d' % (file, a, b, a - b))
        for file, label, got, exp in bad:
            print('  MISMATCH %s: %s: got %s, expected %s' % (file, label, got, exp))
