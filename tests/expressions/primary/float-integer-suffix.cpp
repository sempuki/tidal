// tidal-rules: PRIM-17
// tidal-expect: reject
// tidal-error-clang: invalid suffix .u. on floating constant
// tidal-error-gcc: numeric literal operator .operator.{2}u.
// tidal-error-edg: user-defined literal operator not found
double x = 1.0u;
