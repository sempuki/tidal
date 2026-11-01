// tidal-rules: UNI-13 OBJ-2
// tidal-expect: reject
// tidal-error-clang: read of uninitialized object
// tidal-error-gcc: accessing uninitialized member
// tidal-error-edg: access to uninitialized
struct A { int tag; int a; };
struct B { int tag; int b; };
union U { A a; B b; };
constexpr int read_untouched() { U u{.b = {1, 2}}; u.a.tag = 7; return u.a.a; }
static_assert(read_untouched() == 2);
