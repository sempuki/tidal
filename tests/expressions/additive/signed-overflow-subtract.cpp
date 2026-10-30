// tidal-rules: ADD-9
// tidal-expect: reject
constexpr int int_max = int(~0u >> 1);
constexpr int sub(int a, int b) { return a - b; }
static_assert(sub(-int_max - 1, 1) != 0);
