// tidal-rules: SA-2
// tidal-expect: reject
// tidal-error-gcc: before .?static_assert
// tidal-error-edg: declaration is not valid here
void f() { for (static_assert(true); ;) break; }
