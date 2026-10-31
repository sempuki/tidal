// tidal-rules: SA-1
// tidal-expect: reject
// tidal-error-clang: tidal-marker-message
// tidal-error-gcc: tidal-marker-message
// tidal-error-edg: tidal-marker-message
static_assert(1 == 2, "tidal-marker-message");
