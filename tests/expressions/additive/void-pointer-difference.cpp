// tidal-rules: ADD-3
// tidal-expect: reject
// tidal-error-clang: arithmetic on pointers to void
// tidal-error-gcc: invalid use of .void.
// tidal-error-edg: pointer to a complete object type
long f(void* p, void* q) { return p - q; }
