// tidal-rules: SA-3
// tidal-expect: reject
// tidal-error-clang: not an integral constant expression
// tidal-error-gcc: non-constant condition for static assertion
// tidal-error-edg: must have a constant value
int n = 1;
static_assert(n == 1);
