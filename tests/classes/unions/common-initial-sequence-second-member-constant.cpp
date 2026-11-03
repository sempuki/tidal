// tidal-rules: UNI-12
// tidal-expect: reject
// tidal-error-clang: read of member .* of union with active member
// tidal-error-gcc: member instead of (active|initialized)
// tidal-error-edg: invalid access to inactive member
// tidal-note: Both members of the read are in the common initial sequence. The read is still through the inactive struct.
struct A { int tag; int x; };
struct B { int tag; int y; };
union U { A a; B b; };
constexpr int f() { U u{.a = {1, 7}}; return u.b.y; }
static_assert(f() == 7);
