// tidal-rules: PACK-8
// tidal-expect: reject
// tidal-error-clang: does not contain any unexpanded parameter packs
// tidal-error-gcc: contains no parameter packs
// tidal-error-edg: expected a "\}"
// tidal-note: x is a plain parameter, so the pattern x has no pack.
template <class T> void one(T x) { int y{x...}; }
