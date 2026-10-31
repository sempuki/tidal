// tidal-rules: FN-13
// tidal-expect: reject
// tidal-error-clang: redefinition of parameter .a
// tidal-error-gcc: redefinition of .int a
// tidal-error-edg: duplicate parameter name
void f(int a, int a);
