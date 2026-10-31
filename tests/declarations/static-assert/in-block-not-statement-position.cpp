// tidal-rules: SA-2
// tidal-expect: accept
int f(int v) { if (v) static_assert(true); return v; }
