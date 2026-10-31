// tidal-rules: CE-11
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed
// tidal-error-gcc: narrowing conversion
// tidal-error-edg: narrowing conversion
template <unsigned char N> struct Box {};
Box<300> b;
