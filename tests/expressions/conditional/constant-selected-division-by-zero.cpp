// tidal-rules: COND-15
// tidal-expect: reject
// tidal-error-clang: division by zero
// tidal-error-gcc: is not a constant expression
// tidal-error-edg: division by zero
// tidal-note: The selected operand divides by zero.
constexpr int ratio(int d) { return d != 1 ? 10 / d : 0; }
static_assert(ratio(0) == 0);
