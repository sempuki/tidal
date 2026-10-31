// tidal-rules: INL-2
// tidal-expect: reject
// tidal-error-clang: can only appear on functions and non-local variables
// tidal-error-gcc: declared as an .inline. field
// tidal-error-edg: nonstatic data member cannot be declared as inline
struct S { inline int v = 0; };
