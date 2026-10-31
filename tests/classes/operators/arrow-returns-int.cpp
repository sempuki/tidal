// tidal-rules: OPR-5
// tidal-expect: reject
struct S { int operator->(); };
int f(S s) { return s->x; }
