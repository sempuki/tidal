// tidal-rules: NAM-42
// tidal-expect: reject
// tidal-error-clang: redefinition of .holder. as different kind of symbol
// tidal-error-gcc: conflicting declaration of template
// tidal-error-edg: holder. has already been declared in the current scope
// tidal-note: A class template does not share its name with a function template.
template <class T> struct holder { T value; };
template <class T> void holder(T);
