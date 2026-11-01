// tidal-rules: DED-21 DED-35
// tidal-expect: reject
// tidal-error-clang: deduced conflicting types for parameter
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: The parameter and the result of the function type give T two values.
template <class T> void apply(T (*)(T));
char shrink(int);
void use() { apply(shrink); }
