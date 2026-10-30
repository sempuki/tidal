// tidal-rules: MUL-10
// tidal-expect: reject
constexpr int m(int a, int b) { return a * b; }
static_assert(m(__INT_MAX__, 2) != 0);
