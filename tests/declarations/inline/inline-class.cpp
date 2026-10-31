// tidal-rules: INL-2
// tidal-expect: reject
// tidal-error-clang: can only appear on functions and non-local variables
// tidal-error-gcc: .inline. can only be specified for functions
// tidal-error-edg: inline specifier allowed on function declarations only
inline struct S { int v; };
