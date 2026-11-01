// tidal-rules: TPL-20
// tidal-expect: reject
// tidal-error-clang: template parameter has a different kind in template
// tidal-error-gcc: template parameter .class T.
// tidal-error-edg: declaration is incompatible with template parameter
template <class T> struct Box;
template <int N> struct Box {};
