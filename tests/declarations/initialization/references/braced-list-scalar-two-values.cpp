// tidal-rules: REF-11
// tidal-expect: reject
// tidal-error-clang: excess elements in scalar initializer
// tidal-error-gcc: invalid initialization of reference of type
// tidal-error-edg: too many initializer values
const int& r{1, 2};
