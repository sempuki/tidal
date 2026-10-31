// tidal-rules: INL-2
// tidal-expect: reject
// tidal-error-clang: can only appear on functions and non-local variables
// tidal-error-gcc: declared as an .inline. parameter
// tidal-error-edg: invalid specifier on a parameter
void f(inline int x);
