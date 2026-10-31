// tidal-rules: NAM-3
// tidal-expect: reject
// tidal-error-clang: redefinition of .a. as different kind of symbol
// tidal-error-gcc: redeclared as different kind of entity
// tidal-error-edg: declaration is incompatible with .int a
int a = 1;
void a();
