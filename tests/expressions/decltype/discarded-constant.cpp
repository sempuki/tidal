// tidal-rules: UNEV-2
// tidal-expect: accept
constexpr int f() { int x = 1; x; (void)(x + 1); return x; }
static_assert(f() == 1);
