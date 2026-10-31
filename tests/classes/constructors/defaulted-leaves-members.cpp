// tidal-rules: CTOR-12
// tidal-expect: reject
// tidal-error-gcc: accessing uninitialized member .S::v
// tidal-error-clang: read of uninitialized object
// tidal-error-edg: access to uninitialized object
struct S { int v; };
constexpr int f() { S s; return s.v; }
static_assert(f() == 0);
