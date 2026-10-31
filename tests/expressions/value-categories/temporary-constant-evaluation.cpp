// tidal-rules: VC-8
// tidal-expect: accept
struct S { int v; constexpr int get() const { return v; } };
constexpr int f() { return S{3}.get() + S{4}.v; }
static_assert(f() == 7);
