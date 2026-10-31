// tidal-rules: CE-10
// tidal-expect: reject
// tidal-error-gcc: evaluation depth exceeds maximum
constexpr int depth(int n) { return n == 0 ? 0 : 1 + depth(n - 1); }
static_assert(depth(100000) == 100000);
