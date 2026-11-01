// tidal-rules: TARG-13
// tidal-expect: reject
// tidal-error-clang: is not allowed in a converted constant expression
// tidal-error-gcc: in a converted constant expression
// tidal-error-edg: is incompatible with template parameter of type
// tidal-note: The conversion from int to double is not one that a converted constant expression allows.
template <double D> struct Real {};
Real<1> r;
