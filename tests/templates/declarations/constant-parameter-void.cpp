// tidal-rules: TPL-7
// tidal-expect: reject
// tidal-error-clang: non-type template parameter has incomplete type
// tidal-error-gcc: is not a valid type for a template non-type parameter
// tidal-error-edg: a template parameter may not have void type
template <void V> struct Nothing {};
