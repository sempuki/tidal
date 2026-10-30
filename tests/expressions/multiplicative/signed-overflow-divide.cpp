// tidal-rules: MUL-10
// tidal-expect: reject
constexpr int q(int a, int b) { return a / b; }
static_assert(q(-__INT_MAX__ - 1, -1) != 0);
