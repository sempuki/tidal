// tidal-rules: DED-18 DED-35
// tidal-expect: reject
// tidal-error-clang: couldn't infer template argument
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: A braced list gives a plain T no value.
template <class T> void one(T);
void use() { one({1}); }
