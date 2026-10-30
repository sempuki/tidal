// tidal-rules: COND-1
// tidal-expect: reject
struct S {};
int f() { return S{} ? 1 : 2; }
