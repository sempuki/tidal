// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-error-clang: expected string literal as argument of .deprecated
// tidal-error-gcc: deprecated message is not a string
// tidal-error-edg: expected a string literal
[[deprecated(42)]] int f();
