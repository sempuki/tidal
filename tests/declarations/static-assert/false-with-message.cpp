// tidal-rules: SA-1
// tidal-expect: reject
// tidal-error-clang: static assertion failed
// tidal-error-gcc: static assertion failed
// tidal-error-edg: static assertion failed
static_assert(false, "always fails");
