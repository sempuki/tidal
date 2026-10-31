// tidal-rules: ASN-6
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed
// tidal-error-gcc: narrowing conversion
// tidal-error-edg: narrowing conversion
void f() { int x = 0; x = {2.5}; }
