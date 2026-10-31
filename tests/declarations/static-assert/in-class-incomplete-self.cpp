// tidal-rules: SA-2
// tidal-expect: reject
struct S { int a; static_assert(sizeof(S) > 0); };
