// tidal-rules: ASN-2
// tidal-expect: reject
// tidal-error-clang: array type .* is not assignable
// tidal-error-gcc: invalid array assignment
// tidal-error-edg: must be a modifiable lvalue
void f() { int a[2], b[2]; a = b; }
