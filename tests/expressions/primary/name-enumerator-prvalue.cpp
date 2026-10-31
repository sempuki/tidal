// tidal-rules: PRIM-12 VC-12
// tidal-expect: reject
// tidal-error-clang: cannot take the address of an rvalue of type .E.
// tidal-error-gcc: lvalue required as unary
// tidal-error-edg: must be an lvalue
enum E { a };
const E* p = &a;
