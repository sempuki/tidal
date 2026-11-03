// tidal-rules: NAM-42
// tidal-expect: reject
// tidal-error-clang: redefinition of .width. as different kind of symbol
// tidal-error-gcc: redeclared as different kind of entity
// tidal-error-edg: width. has already been declared in the current scope
// tidal-note: A class does not share its name with a variable template.
struct width { int v; };
template <class T> int width = sizeof(T);
