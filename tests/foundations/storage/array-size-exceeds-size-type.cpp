// tidal-rules: STG-1 DCL-19
// tidal-expect: reject
// tidal-error-gcc: exceeds maximum object size
// tidal-error-clang: array is too large
// tidal-error-edg: array is too large
char table[(decltype(sizeof 0))-1][2];
