// tidal-rules: TARG-15
// tidal-expect: reject
// tidal-error-clang: in converted constant expression would bind reference to a
// tidal-error-gcc: in converted constant expression does not bind directly
// tidal-error-edg: expression must have a constant value
template <const int& R> struct Ref {};
Ref<5> r;
