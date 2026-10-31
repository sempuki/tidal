// tidal-rules: NEW-17
// tidal-expect: reject
// tidal-error-clang: array is too large
// tidal-error-gcc: size .-1. of array is negative
// tidal-error-edg: must be non-negative
void f() { int* p = new int[-1]; }
