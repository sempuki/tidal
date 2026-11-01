// tidal-rules: PRIM-29
// tidal-expect: reject
// tidal-error-clang: expected expression
// tidal-error-gcc: before .<=>. token
// tidal-error-edg: invalid fold expression operator
template <int... V> constexpr auto order = (V <=> ...);
