// tidal-rules: PFX-13
// tidal-expect: reject
// tidal-error-clang: arithmetic on a pointer to void
// tidal-error-gcc: incrementing a pointer of type .void\*.
// tidal-error-edg: pointer to a complete object type
void f(void* p) { p++; }
