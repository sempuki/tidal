// tidal-rules: DED-10 DED-35
// tidal-expect: reject
// tidal-error-clang: could not match
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: No value of T makes T* the type int.
template <class T> void point(T*);
void use() { point(1); }
