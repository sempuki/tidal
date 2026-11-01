// tidal-rules: INST-25
// tidal-expect: reject
// tidal-error-clang: explicit instantiation cannot be 'constexpr'
// tidal-error-gcc: explicit instantiation shall not use .constexpr. specifier
// tidal-error-edg: "constexpr" is not allowed on an explicit instantiation
template <class T> constexpr int get(T) { return 1; }
template constexpr int get(int);
