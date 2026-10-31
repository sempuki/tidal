// tidal-rules: ENUM-2
// tidal-expect: reject
// tidal-error-clang: redefinition of .Red
// tidal-error-gcc: Red. redeclared as different kind of entity
// tidal-error-edg: Red. has already been declared
int Red = 0;
enum Color { Red };
