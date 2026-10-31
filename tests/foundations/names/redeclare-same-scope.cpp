// tidal-rules: NAM-3
// tidal-expect: reject
// tidal-error-clang: redefinition of .a
// tidal-error-gcc: redeclaration of .int a
// tidal-error-edg: a. has already been declared in the current scope
int main() { int a = 1; int a = 2; return a; }
