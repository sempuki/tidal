// tidal-rules: MUL-9
// tidal-expect: reject
constexpr int r(int a, int b) { return a % b; }
static_assert(r(1, 0) == 0);
