// tidal-rules: PRIM-26
// tidal-expect: reject
// tidal-error-clang: invalid suffix .\.3. on floating constant
// tidal-error-gcc: too many decimal points in number
// tidal-error-edg: extra text after expected end of number
double x = 1.2.3;
