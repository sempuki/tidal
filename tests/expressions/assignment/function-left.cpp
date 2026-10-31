// tidal-rules: ASN-2
// tidal-expect: reject
// tidal-error-clang: non-object type .* is not assignable
// tidal-error-gcc: assignment of function
// tidal-error-edg: must be a modifiable lvalue
void g();
void f() { g = nullptr; }
