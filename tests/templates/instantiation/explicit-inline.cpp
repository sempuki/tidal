// tidal-rules: INST-25
// tidal-expect: reject
// tidal-error-clang: explicit instantiation cannot be 'inline'
// tidal-error-gcc: explicit instantiation shall not use .inline. specifier
// tidal-error-edg: "inline" is not allowed
template <class T> int get(T) { return 1; }
template inline int get(int);
