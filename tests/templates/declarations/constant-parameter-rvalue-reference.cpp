// tidal-rules: TPL-7
// tidal-expect: reject
// tidal-error-clang: non-type template parameter has rvalue reference type
// tidal-error-gcc: is not a valid type for a template non-type parameter
// tidal-error-edg: a nontype template parameter cannot have rvalue reference
template <int&& R> struct Ref {};
