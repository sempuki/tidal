// tidal-rules: TYN-2
// tidal-expect: reject
// tidal-error-clang: redefinition of .X. as different kind of symbol
// tidal-error-gcc: redeclared as different kind of entity
// tidal-error-edg: has already been declared in the current scope
int X;
typedef int X;
