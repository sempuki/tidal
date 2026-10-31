// tidal-rules: BIT-3
// tidal-expect: reject
// tidal-error-clang: invalid argument type .int \*. to unary expression
// tidal-error-gcc: wrong type argument to bit-complement
// tidal-error-edg: must have integral or unscoped enum type
int x;
auto r = ~&x;
