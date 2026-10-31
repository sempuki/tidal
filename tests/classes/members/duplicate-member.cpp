// tidal-rules: CLS-9
// tidal-expect: reject
// tidal-error-clang: duplicate member .a
// tidal-error-gcc: redeclaration of .double S::a
// tidal-error-edg: has already been declared in the current scope
struct S { int a; double a; };
