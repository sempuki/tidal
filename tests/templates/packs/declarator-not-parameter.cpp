// tidal-rules: PACK-2
// tidal-expect: reject
// tidal-error-clang: only function and template parameters can be parameter packs
// tidal-error-gcc: expected unqualified-id before .\.\.\.
// tidal-error-edg: expected an identifier
// tidal-note: A local variable cannot be declared with an ellipsis.
template <class... T> void make() { T... values; }
