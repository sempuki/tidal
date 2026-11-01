// tidal-rules: CPT-18
// tidal-expect: reject
// tidal-error-clang: not a type concept
// tidal-error-gcc: does not constrain a type
// tidal-error-edg: type constraint uses
template <int N> concept Positive = N > 0;
Positive auto value = 1;
