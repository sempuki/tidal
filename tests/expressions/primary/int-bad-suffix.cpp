// tidal-rules: PRIM-16
// tidal-expect: reject
// tidal-error-clang: invalid suffix .lul. on integer constant
// tidal-error-gcc: numeric literal operator .operator.{2}lul.
// tidal-error-edg: user-defined literal operator not found
auto x = 1lul;
