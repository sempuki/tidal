// tidal-rules: ASN-2
// tidal-expect: reject
// tidal-error-clang: const-qualified type
// tidal-error-gcc: assignment of read-only variable
// tidal-error-edg: must be a modifiable lvalue
void f() { const int c = 1; c = 2; }
