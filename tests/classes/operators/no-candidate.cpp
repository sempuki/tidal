// tidal-rules: OPR-6
// tidal-expect: reject
struct S {};
int f(S a, S b) { return a + b ? 1 : 0; }
