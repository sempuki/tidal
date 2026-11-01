// tidal-rules: PRIM-29
// tidal-expect: reject
// tidal-error-clang: unexpanded parameter pack
// tidal-error-gcc: parameter packs not expanded
// tidal-error-edg: referenced but not expanded
template <int... V> constexpr int bare() { return V + ...; }
int total = bare<1, 2>();
