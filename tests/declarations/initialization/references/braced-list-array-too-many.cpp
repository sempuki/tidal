// tidal-rules: REF-11
// tidal-expect: reject
// tidal-error-clang: excess elements in array initializer
// tidal-error-gcc: invalid initialization of reference of type
// tidal-error-edg: too many initializer values
const int (&values)[2]{1, 2, 3};
