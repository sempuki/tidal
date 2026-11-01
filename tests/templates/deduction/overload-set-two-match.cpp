// tidal-rules: DED-19 DED-35
// tidal-expect: reject
// tidal-error-clang: couldn't infer template argument
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: Both functions of the set match T (*)(T), so the pair deduces nothing and T has no value.
template <class T> void apply(T (*)(T));
int square(int);
long square(long);
void use() { apply(square); }
