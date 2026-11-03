// tidal-rules: TYN-2
// tidal-expect: reject
// tidal-error-clang: redefinition of .span. as different kind of symbol
// tidal-error-gcc: redeclared as different kind of entity
// tidal-error-edg: span. has already been declared in the current scope
// tidal-note: The typedef names the class with its own name. A variable with that name conflicts with the typedef, although the class alone could share the name.
typedef struct span { int v; } span;
int span = 1;
