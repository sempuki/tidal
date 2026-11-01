// tidal-rules: TSP-8
// tidal-expect: reject
// tidal-error-clang: default argument not permitted on an explicit
// tidal-error-gcc: default argument specified in explicit specialization
// tidal-error-edg: default argument is not allowed
template <class T> int scale(T v, int by) { return v * by; }
template <> int scale<int>(int v, int by = 2) { return v * by; }
