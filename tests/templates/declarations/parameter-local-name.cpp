// tidal-rules: TPL-18
// tidal-expect: reject
// tidal-error-clang: shadows template parameter
// tidal-error-gcc: shadows template parameter
// tidal-error-edg: may not be redeclared in this scope
template <class T> int use() { int T = 0; return T; }
