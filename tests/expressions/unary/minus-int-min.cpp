// tidal-rules: UN-4
// tidal-expect: reject
constexpr int neg(int v) { return -v; }
static_assert(neg(-2147483647 - 1) != 0);
