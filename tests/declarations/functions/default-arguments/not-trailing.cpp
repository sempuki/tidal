// tidal-rules: DEFARG-8
// tidal-expect: reject
// tidal-error-clang: missing default argument on parameter
// tidal-error-gcc: default argument missing for parameter
// tidal-error-edg: default argument not at end of parameter list
int f(int a = 1, int b);
