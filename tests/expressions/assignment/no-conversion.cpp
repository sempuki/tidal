// tidal-rules: ASN-1
// tidal-expect: reject
// tidal-error-clang: incompatible integer to pointer conversion
// tidal-error-gcc: invalid conversion from .int. to .int\*.
// tidal-error-edg: cannot be assigned to an entity of type
void f(int* p) { p = 1; }
