// tidal-rules: PRIM-18
// tidal-expect: reject
// tidal-error-clang: invalid universal character
// tidal-error-gcc: is not a valid universal character
// tidal-error-edg: cannot designate a surrogate code point
auto c = U'\uD800';
