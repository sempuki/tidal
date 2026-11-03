// tidal-rules: NAM-42
// tidal-expect: reject
// tidal-error-clang: redefinition of .holder. as different kind of symbol
// tidal-error-gcc: redeclared as different kind of entity
// tidal-error-edg: holder. has already been declared in the current scope
// tidal-note: The variable comes first and the class template second.
int holder = 1;
template <class T> struct holder { T value; };
