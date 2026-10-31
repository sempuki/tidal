// tidal-rules: CTOR-12
// tidal-expect: reject
// tidal-error-gcc: accessing uninitialized member .D::m
// tidal-note: Gap review 2026-10-31.
struct A { int a; constexpr A(int v) : a(v) {} };
struct D : A { int m = 5; constexpr int get() const { return m; } constexpr D() : A(get()) {} };
constexpr D d;
