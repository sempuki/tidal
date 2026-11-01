// tidal-rules: NEW-17
// tidal-expect: reject
// tidal-error-clang: array is too large
// tidal-error-gcc: exceeds maximum object size
// tidal-error-edg: array is too large
int* make() { return new int[decltype(sizeof 0)(-1) / 2]; }
