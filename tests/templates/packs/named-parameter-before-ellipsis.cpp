// tidal-rules: PACK-3 PACK-8
// tidal-expect: reject
// tidal-error-clang: must immediately precede declared identifier
// tidal-error-gcc: parameter packs not expanded
// tidal-error-edg: parameter pack "T" was referenced but not expanded
// tidal-note: The parameter x has a name, so the ellipsis after it is the variable argument part, and the pack T in the type of x stays unexpanded.
template <class... T> void take(T x...);
