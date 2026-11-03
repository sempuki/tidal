// tidal-rules: TYN-2
// tidal-expect: reject
// tidal-error-clang: redefinition of .span. as different kind of symbol
// tidal-error-gcc: redeclared as different kind of entity
// tidal-error-edg: span. has already been declared in the current scope
// tidal-note: The typedef comes first and the variable second. A class could share the name with the variable, and a typedef cannot.
typedef int span;
int span = 1;
