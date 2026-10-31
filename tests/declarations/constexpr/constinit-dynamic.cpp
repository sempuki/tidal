// tidal-rules: CINIT-1
// tidal-expect: reject
// tidal-error-clang: variable does not have a constant initializer
// tidal-error-gcc: constinit. variable .value. does not have a constant initializer
// tidal-error-edg: must have a constant value
int read();
constinit int value = read();
