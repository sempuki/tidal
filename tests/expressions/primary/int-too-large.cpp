// tidal-rules: PRIM-2
// tidal-expect: reject
// tidal-error-clang: too large to be represented in any integer type
// tidal-error-gcc: integer constant is too large for its type
// tidal-error-edg: integer constant is too large
auto x = 18446744073709551616;
