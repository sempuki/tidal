// tidal-rules: DCL-5
// tidal-expect: reject
// tidal-error-clang: excess elements in array initializer
// tidal-error-gcc: too many initializers
// tidal-error-edg: too many initializer values
int a[2] = {1, 2, 3};
