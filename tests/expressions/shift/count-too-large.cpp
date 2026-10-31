// tidal-rules: SHF-7
// tidal-expect: reject
// tidal-error-gcc: greater than or equal to the precision
constexpr int sh(int v, int n) { return v << n; }
static_assert(sh(1, 32) == 0);
