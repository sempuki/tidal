// tidal-rules: DCL-3
// tidal-expect: reject
// tidal-error-clang: cannot form a reference to .void
// tidal-error-gcc: cannot declare reference to .void
// tidal-error-edg: reference to void is not allowed
void& r;
