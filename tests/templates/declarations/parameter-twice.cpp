// tidal-rules: TPL-18
// tidal-expect: reject
// tidal-error-clang: shadows template parameter
// tidal-error-gcc: conflicting declaration
// tidal-error-edg: may not be redeclared in this scope
template <class T, class T> struct Pair {};
