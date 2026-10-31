// tidal-rules: CV-8
// tidal-expect: reject
// tidal-error-clang: variable length array
// tidal-error-gcc: size of array .b. is not an integral constant-expression
// tidal-error-edg: must have a constant value
int read();
const int m = read();
int b[m];
