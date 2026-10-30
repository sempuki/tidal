// tidal-rules: PFX-11
// tidal-expect: reject
struct S { int v; };
int f(S s) { return s.w; }
