// tidal-rules: ALN-2
// tidal-expect: reject
// tidal-error-clang: cannot be applied to a function parameter
// tidal-error-gcc: alignment may not be specified
// tidal-error-edg: alignas does not apply here
void f(alignas(16) int x);
