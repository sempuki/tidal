// tidal-rules: SHF-7
// tidal-expect: reject
// tidal-error-gcc: greater than or equal to the precision
// tidal-error-clang: shift count 32 >= width of type
// tidal-error-edg: shift count is too large
constexpr int sh(int v, int n) { return v << n; }
static_assert(sh(1, 32) == 0);
