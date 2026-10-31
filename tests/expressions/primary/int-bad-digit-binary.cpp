// tidal-rules: PRIM-16
// tidal-expect: reject
// tidal-error-clang: invalid digit .2. in binary constant
// tidal-error-gcc: invalid digit .2. in binary constant
// tidal-error-edg: invalid binary number
int x = 0b102;
