// tidal-rules: CE-1
// tidal-expect: accept
constexpr int square(int v) { return v * v; }
constexpr int sum(int n) { int t = 0; for (int i = 1; i <= n; ++i) t += square(i); return t; }
static_assert(sum(3) == 14);
constexpr int k = square(4) + 1;
static_assert(k == 17);
