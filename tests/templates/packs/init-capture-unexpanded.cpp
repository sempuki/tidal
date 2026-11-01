// tidal-rules: PACK-8 PACK-18
// tidal-expect: reject
// tidal-error-clang: contains unexpanded parameter pack
// tidal-error-gcc: parameter packs not expanded
// tidal-error-edg: was referenced but not expanded
// tidal-note: The init capture y is a pack, so the body must expand it.
template <class... T> void keep(T... x) { auto f = [...y = x] { return y; }; }
