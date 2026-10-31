// tidal-rules: SHF-7
// tidal-expect: reject
// tidal-error-gcc: greater than or equal to the precision
constexpr unsigned sh(unsigned v, int n) { return v >> n; }
static_assert(sh(1u, 32) == 0);
