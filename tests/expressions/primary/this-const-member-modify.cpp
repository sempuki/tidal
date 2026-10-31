// tidal-rules: PRIM-11 ASN-2
// tidal-expect: reject
// tidal-error-clang: within const member function
// tidal-error-gcc: in read-only object
// tidal-error-edg: must be a modifiable lvalue
struct S { int v; void f() const { v = 1; } };
