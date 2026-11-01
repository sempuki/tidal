// tidal-rules: TPL-18
// tidal-expect: reject
// tidal-error-clang: shadows template parameter
// tidal-error-gcc: shadows template parameter
// tidal-error-edg: may not be redeclared in this scope
template <class T> struct Box { int T; };
