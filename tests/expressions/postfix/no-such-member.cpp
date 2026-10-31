// tidal-rules: PFX-11
// tidal-expect: reject
// tidal-error-clang: no member named .w. in .S.
// tidal-error-gcc: has no member named .w.
// tidal-error-edg: has no member .w.
struct S { int v; };
int f(S s) { return s.w; }
