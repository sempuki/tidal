// tidal-rules: BIT-3
// tidal-expect: reject
// tidal-error-clang: invalid argument type .double. to unary expression
// tidal-error-gcc: wrong type argument to bit-complement
// tidal-error-edg: must have integral or unscoped enum type
auto r = ~1.0;
