// tidal-rules: LOG-7
// tidal-expect: accept
constexpr bool f(int d) { return d != 0 && 10 / d > 1; }
constexpr bool g(int d) { return d == 0 || 10 / d > 1; }
static_assert(!f(0) && f(2));
static_assert(g(0));
