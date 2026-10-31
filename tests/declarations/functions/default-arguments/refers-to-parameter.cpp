// tidal-rules: DEFARG-4
// tidal-expect: reject
// tidal-error-clang: default argument references parameter
// tidal-error-gcc: parameter .a. may not appear in this context
// tidal-error-edg: parameter is not allowed
int f(int a, int b = a);
