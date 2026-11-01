// tidal-rules: DED-23 DED-35
// tidal-expect: reject
// tidal-error-clang: couldn't infer template argument
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: U appears in no parameter and has no default.
template <class T, class U> U make(T);
void use() { make(1); }
