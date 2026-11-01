// tidal-rules: TARG-13
// tidal-expect: reject
// tidal-error-clang: is not allowed in a converted constant expression
// tidal-error-gcc: in a converted constant expression
// tidal-error-edg: is incompatible with template parameter of type
template <int N> struct Num {};
Num<2.0> n;
