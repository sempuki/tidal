// tidal-rules: TPL-20
// tidal-expect: reject
// tidal-error-clang: template type parameter pack conflicts with previous template type parameter
// tidal-error-gcc: template parameter .class T.
// tidal-error-edg: declaration is incompatible with
template <class T> struct Box;
template <class... T> struct Box {};
