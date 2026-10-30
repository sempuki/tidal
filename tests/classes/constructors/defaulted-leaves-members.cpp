// tidal-rules: CTOR-4
// tidal-expect: reject
struct S { int v; };
constexpr int f() { S s; return s.v; }
static_assert(f() == 0);
