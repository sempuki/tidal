// tidal-rules: PFX-11
// tidal-expect: reject
struct S { int v; };
int f(S* p) { return p.v; }
