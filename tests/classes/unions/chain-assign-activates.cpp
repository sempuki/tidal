// tidal-rules: UNI-13
// tidal-expect: accept
struct A { int tag; int a; };
struct B { int tag; double b; };
union U { A a; B b; };
struct Holder { int k; U u; };
union Outer { U inner; int i; };
union Numbers { int arr[2]; long l; };
constexpr int from_none() { U u; u.a.tag = 7; u.a.a = 1; return u.a.tag + u.a.a; }
constexpr int from_other() { U u{}; u.a.tag = 1; u.b.tag = 3; return u.b.tag; }
constexpr int inside_class() { Holder h{}; h.u.b.tag = 2; return h.u.b.tag; }
constexpr int two_unions() { Outer o{}; o.inner.b.tag = 4; return o.inner.b.tag; }
constexpr int through_subscript() { Numbers n{}; n.l = 1; n.arr[1] = 9; return n.arr[1]; }
static_assert(from_none() == 8);
static_assert(from_other() == 3);
static_assert(inside_class() == 2);
static_assert(two_unions() == 4);
static_assert(through_subscript() == 9);
