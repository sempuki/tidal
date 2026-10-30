// tidal-rules: MUL-10
// tidal-expect: reject
constexpr int r(int a, int b) { return a % b; }
static_assert(r(-__INT_MAX__ - 1, -1) == 0);
