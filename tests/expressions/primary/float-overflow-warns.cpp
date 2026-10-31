// tidal-rules: PRIM-13
// tidal-expect: reject
// tidal-error-clang: magnitude of floating-point constant too large
// tidal-error-gcc: floating constant exceeds range
// tidal-flags: -Werror
float x = 1e999f;
