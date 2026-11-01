// tidal-rules: PRIM-29
// tidal-expect: reject
// tidal-error-clang: does not contain any unexpanded parameter packs
// tidal-error-gcc: has no unexpanded parameter packs
// tidal-error-edg: does not refer to a parameter pack
template <int N> constexpr int single = (N + ...);
