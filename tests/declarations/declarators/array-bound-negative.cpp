// tidal-rules: DCL-5
// tidal-expect: reject
// tidal-error-clang: declared as an array with a negative size
// tidal-error-gcc: narrowing conversion of .-1.
// tidal-error-edg: size of an array must be greater than zero
int a[-1];
