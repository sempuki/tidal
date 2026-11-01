// tidal-rules: PACK-8
// tidal-expect: reject
// tidal-error-clang: contains unexpanded parameter pack
// tidal-error-gcc: parameter packs not expanded
// tidal-error-edg: was referenced but not expanded
template <class... T> void first(T... x) { int y = x; }
