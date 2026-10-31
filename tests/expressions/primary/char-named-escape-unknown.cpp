// tidal-rules: PRIM-18
// tidal-expect: reject
// tidal-error-clang: is not a valid Unicode character name
// tidal-error-gcc: is not a valid universal character
// tidal-error-edg: does not designate a known Unicode character
// tidal-note: Gap review 2026-10-31.
auto c = U'\N{NO SUCH CHARACTER NAME}';
