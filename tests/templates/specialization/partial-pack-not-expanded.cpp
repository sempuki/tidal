// tidal-rules: TSP-11
// tidal-expect: reject
// tidal-error-clang: unexpanded parameter pack
// tidal-error-gcc: parameter packs not expanded
// tidal-error-edg: parameter pack "T" was referenced but not expanded
template <class... A> struct List { static constexpr int kind = 0; };
template <class... T> struct List<T, int> { static constexpr int kind = 1; };
