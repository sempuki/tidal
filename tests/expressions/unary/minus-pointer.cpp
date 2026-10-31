// tidal-rules: UN-2
// tidal-expect: reject
// tidal-error-clang: invalid argument type .int \*. to unary expression
// tidal-error-gcc: wrong type argument to unary minus
// tidal-error-edg: must have arithmetic or unscoped enum type
int x;
auto p = -&x;
