// tidal-rules: PRIM-11
// tidal-expect: reject
struct S { static S* f() { return this; } };
