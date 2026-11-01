// tidal-rules: TPL-7
// tidal-expect: reject
// tidal-error-clang: non-type template parameter has incomplete type 'void'
// tidal-error-gcc: void. is not a valid type for a template non-type parameter
// tidal-error-edg: cannot deduce "auto" template parameter type
template <auto V> struct Hold {};
Hold<(void)0> h;
