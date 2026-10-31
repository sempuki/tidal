// tidal-rules: SHF-7
// tidal-expect: reject
// tidal-error-gcc: right operand of shift expression .* is negative
constexpr int sh(int v, int n) { return v << n; }
static_assert(sh(1, -1) == 0);
