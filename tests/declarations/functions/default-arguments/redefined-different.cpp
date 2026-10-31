// tidal-rules: DEFARG-9
// tidal-expect: reject
// tidal-error-clang: redefinition of default argument
// tidal-error-gcc: default argument given for parameter 1
// tidal-error-edg: redefinition of default argument
int f(int a = 1);
int f(int a = 2);
