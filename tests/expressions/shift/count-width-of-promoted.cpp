// tidal-rules: SHF-7
// tidal-expect: accept
constexpr int sh(char c, int n) { return c << n; }
static_assert(sh(1, 20) == 1 << 20);
