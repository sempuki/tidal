// tidal-rules: SA-5
// tidal-expect: reject
int f() { if constexpr (false) { static_assert(false); } return 0; }
