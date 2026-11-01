// tidal-rules: TPL-34
// tidal-expect: reject
// tidal-error-clang: template parameter pack cannot have a default argument
// tidal-error-gcc: template parameter pack .C. cannot have a default argument
// tidal-error-edg: a parameter pack cannot have a default
template <class> struct Box {};
template <template <class> class... C = Box> struct Use {};
