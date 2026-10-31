// tidal-rules: SA-7
// tidal-expect: reject
// tidal-error-clang: not an integral constant expression
// tidal-error-gcc: division by zero
// tidal-error-edg: must have a constant value
static_assert(1 / 0 == 0);
