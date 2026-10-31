// tidal-rules: SA-5
// tidal-expect: reject
// tidal-error-clang: static assertion failed
// tidal-error-gcc: static assertion failed
// tidal-error-edg: static assertion failed
int f() { if constexpr (false) { static_assert(false); } return 0; }
