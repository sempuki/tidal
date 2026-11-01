// tidal-rules: TPL-20
// tidal-expect: reject
// tidal-error-clang: template non-type parameter has a different type
// tidal-error-gcc: template parameter .int N.
// tidal-error-edg: declaration is incompatible with constant
template <int N> struct Box;
template <long N> struct Box {};
