// tidal-rules: DCL-10
// tidal-expect: reject
// tidal-error-clang: array is too large
// tidal-error-gcc: narrowing conversion of .4294967296
// tidal-flags: -m32
char big[1ull << 32];
