// tidal-rules: UNI-10
// tidal-expect: reject
// tidal-error-gcc: member instead of (active|initialized)
// tidal-error-clang: read of member .* of union with active member
// tidal-error-edg: invalid access to inactive member
struct A { int tag; int a; };
struct B { int tag; double b; };
union U { A a; B b; };
constexpr int f() { U u{.a = {7, 1}}; return u.b.tag; }
static_assert(f() == 7);
