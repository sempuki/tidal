// tidal-rules: LOG-14
// tidal-expect: reject
// tidal-error-clang: division by zero
// tidal-error-gcc: 10 / 0.* is not a constant expression
// tidal-error-edg: division by zero
constexpr bool fits(int d) { return d == 1 || 10 / d > 1; }
static_assert(fits(0));
