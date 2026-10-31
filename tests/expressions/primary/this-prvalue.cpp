// tidal-rules: PRIM-11
// tidal-expect: reject
struct S { void f() { S** p = &this; } };
