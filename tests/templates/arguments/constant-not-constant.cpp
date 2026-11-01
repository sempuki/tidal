// tidal-rules: TARG-13
// tidal-expect: reject
// tidal-error-clang: non-type template argument is not a constant expression
// tidal-error-gcc: is not usable in a constant expression
// tidal-error-edg: expression must have a constant value
template <int N> struct Num {};
int count = 3;
Num<count> n;
