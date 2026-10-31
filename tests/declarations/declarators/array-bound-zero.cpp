// tidal-rules: DCL-5
// tidal-expect: reject
// tidal-error-clang: zero size arrays
// tidal-error-gcc: forbids zero-size array
// tidal-error-edg: size of an array must be greater than zero
int a[0];
