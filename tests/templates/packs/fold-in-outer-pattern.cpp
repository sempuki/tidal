// tidal-rules: PACK-8
// tidal-expect: reject
// tidal-error-clang: does not contain any unexpanded parameter packs
// tidal-error-gcc: contains no parameter packs
// tidal-error-edg: pack expansion does not make use of any argument packs
// tidal-note: The fold (N + ...) uses up the pack N, so the outer pattern has no pack of its own.
template <class... T> int take(T...);
template <int... N> void sum() { take((N + ...)...); }
