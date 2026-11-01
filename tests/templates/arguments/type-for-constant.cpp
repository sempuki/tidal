// tidal-rules: TARG-7
// tidal-expect: reject
// tidal-error-clang: template argument for non-type template parameter must be
// tidal-error-gcc: type/value mismatch at argument 1 in template parameter
// tidal-error-edg: type name is not allowed
template <int N> struct Num {};
Num<int> n;
