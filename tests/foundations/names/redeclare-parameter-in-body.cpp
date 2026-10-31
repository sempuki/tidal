// tidal-rules: NAM-3
// tidal-expect: reject
// tidal-error-clang: redefinition of .a
// tidal-error-gcc: declaration of .int a. shadows a parameter
// tidal-error-edg: a. has already been declared in the current scope
int f(int a) { int a = 2; return a; }
