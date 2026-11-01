// tidal-rules: DED-9 DED-35
// tidal-expect: reject
// tidal-error-clang: couldn't infer template argument
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: The call uses the default argument, so the parameter takes no part, and T has no value.
template <class T> void fill(T value = 0) {}
void use() { fill(); }
