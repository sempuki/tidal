// tidal-rules: SA-2
// tidal-expect: reject
void f() { for (static_assert(true); ;) break; }
