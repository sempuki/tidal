// tidal-rules: CE-10
// tidal-expect: accept
constexpr int depth(int n) { return n == 0 ? 0 : 1 + depth(n - 1); }
static_assert(depth(400) == 400);
