// tidal-rules: TPL-34
// tidal-expect: reject
// tidal-error-clang: template parameter pack cannot have a default argument
// tidal-error-gcc: template parameter pack .N. cannot have a default argument
// tidal-error-edg: a parameter pack cannot have a default
template <int... N = 1> int count() { return sizeof...(N); }
