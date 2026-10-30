// tidal-rules: MUL-10
// tidal-expect: reject
constexpr int int_max = int(~0u >> 1);
constexpr int r(int a, int b) { return a % b; }
static_assert(r(-int_max - 1, -1) == 0);
