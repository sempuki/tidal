// tidal-rules: PRIM-17
// tidal-expect: reject
// tidal-error-clang: hexadecimal floating literal requires an exponent
// tidal-error-gcc: hexadecimal floating constants require an exponent
// tidal-error-edg: invalid floating constant
double x = 0x1.8;
