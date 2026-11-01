// tidal-rules: PACK-2
// tidal-expect: reject
// tidal-error-clang: does not contain any unexpanded parameter packs
// tidal-error-gcc: contains no parameter packs
// tidal-error-edg: expected a "\)"
// tidal-note: The type int names no pack and is no placeholder, so int... x is not a function parameter pack.
void count(int... x);
