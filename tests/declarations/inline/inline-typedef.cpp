// tidal-rules: INL-2
// tidal-expect: reject
// tidal-error-clang: can only appear on functions and non-local variables
// tidal-error-gcc: declared as an .inline. type
// tidal-error-edg: inline specifier allowed on function declarations only
inline typedef int I;
