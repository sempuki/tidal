// tidal-rules: OBJ-2
// tidal-expect: reject
constexpr int f() { int x; return x; }
static_assert(f() == 0);
