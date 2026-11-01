// tidal-rules: DED-19 DED-35
// tidal-expect: reject
// tidal-error-clang: couldn't infer template argument
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: The set holds a function template, so the pair deduces nothing.
template <class T> void apply(T (*)(int));
int make(int);
template <class U> U make(U);
void use() { apply(make); }
