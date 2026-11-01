// tidal-rules: TPL-36 TPL-30
// tidal-expect: reject
// tidal-error-clang: template parameter pack must be the last template parameter
// tidal-error-gcc: must be at the end of the template parameter list
// tidal-error-edg: template parameter pack not at end of parameter list
template <class... T, T... V> struct Values {};
