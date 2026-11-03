// tidal-rules: NAM-42
// tidal-expect: reject
// tidal-error-clang: redefinition of .holder. as different kind of symbol
// tidal-error-gcc: redeclared as different kind of entity
// tidal-error-edg: holder. has already been declared in the current scope
// tidal-note: A class template does not share its name with a variable.
template <class T> struct holder { T value; };
int holder = 1;
