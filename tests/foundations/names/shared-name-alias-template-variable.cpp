// tidal-rules: NAM-42
// tidal-expect: reject
// tidal-error-clang: redefinition of .ptr. as different kind of symbol
// tidal-error-gcc: redeclared as different kind of entity
// tidal-error-edg: ptr. has already been declared in the current scope
// tidal-note: An alias template does not share its name with a variable.
template <class T> using ptr = T*;
int ptr = 1;
