// tidal-rules: CV-11
// tidal-expect: reject
// tidal-error-clang: not an integral constant expression
// tidal-error-gcc: value of .d. is not usable in a constant expression
// tidal-error-edg: must have a constant value
const double d = 2.0;
static_assert(d == 2.0);
