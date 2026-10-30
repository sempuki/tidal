// tidal-rules: SHF-7
// tidal-expect: reject
constexpr int sh(int v, int n) { return v << n; }
static_assert(sh(1, 32) == 0);
