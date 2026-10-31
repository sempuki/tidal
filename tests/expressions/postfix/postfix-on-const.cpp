// tidal-rules: PFX-13
// tidal-expect: reject
// tidal-error-clang: const-qualified type
// tidal-error-gcc: increment of read-only parameter
// tidal-error-edg: must be a modifiable lvalue
void f(const int i) { i++; }
