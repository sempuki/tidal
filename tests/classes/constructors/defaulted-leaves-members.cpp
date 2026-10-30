// tidal-rules: CTOR-12
// tidal-expect: reject
struct S { int v; };
constexpr int f() { S s; return s.v; }
static_assert(f() == 0);
