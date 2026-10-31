// tidal-rules: SA-3
// tidal-expect: accept
constexpr int sq(int v) { return v * v; }
static_assert(sq(4) == 16);
