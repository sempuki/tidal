// tidal-rules: DED-13 DED-35
// tidal-expect: reject
// tidal-error-clang: couldn't infer template argument
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: The operand of decltype is a non-deduced position.
template <class T> void typed(decltype(T{}));
void use() { typed(1); }
