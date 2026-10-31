// tidal-rules: PRIM-11 VC-12
// tidal-expect: reject
// tidal-error-clang: cannot take the address of an rvalue of type .S \*.
// tidal-error-gcc: lvalue required as unary
// tidal-error-edg: must be an lvalue
struct S { void f() { S** p = &this; } };
