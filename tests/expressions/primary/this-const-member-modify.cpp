// tidal-rules: PRIM-11
// tidal-expect: reject
struct S { int v; void f() const { v = 1; } };
