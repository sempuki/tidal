// tidal-rules: DED-12 DED-35
// tidal-expect: reject
// tidal-error-clang: does not have the same type as the corresponding template parameter
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: Fixed takes an int, and the parameter N of take is a long, so deduction fails.
template <int N> struct Fixed {};
template <long N> void take(Fixed<N>);
void use() { take(Fixed<4>{}); }
