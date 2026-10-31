// tidal-rules: ADD-3
// tidal-expect: reject
// tidal-error-clang: arithmetic on a pointer to void
// tidal-error-gcc: pointer of type .void \*. used in arithmetic
// tidal-error-edg: pointer to a complete object type
void* f(void* p) { return p + 1; }
