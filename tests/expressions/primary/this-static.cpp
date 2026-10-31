// tidal-rules: PRIM-20
// tidal-expect: reject
struct S { static S* f() { return this; } };
