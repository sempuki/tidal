// tidal-rules: TARG-15
// tidal-expect: reject
// tidal-error-clang: is not allowed in a converted constant expression
// tidal-error-gcc: in a converted constant expression
// tidal-error-edg: is incompatible with template parameter of type
// tidal-note: 0 is an int, and an int does not convert to a pointer in a converted constant expression.
template <int* P> struct Ptr {};
Ptr<0> p;
