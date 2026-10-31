// tidal-rules: NAM-8
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .a
// tidal-error-gcc: a. was not declared in this scope
// tidal-error-edg: identifier .a. is undefined
int main() { int b = a; int a = 1; return b; }
