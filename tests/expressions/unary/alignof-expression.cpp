// tidal-rules: UN-17
// tidal-expect: reject
// tidal-error-clang: alignof. applied to an expression
// tidal-error-gcc: alignof. with a non-type
// tidal-error-edg: does not accept an expression argument
int x;
auto a = alignof(x);
