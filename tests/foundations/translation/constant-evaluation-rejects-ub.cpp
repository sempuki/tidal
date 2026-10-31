// tidal-rules: TR-5
// tidal-expect: reject
constexpr int f(int d) { return 1 / d; }
static_assert(f(0) == 0);
