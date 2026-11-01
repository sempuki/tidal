// tidal-rules: DED-13 DED-35
// tidal-expect: reject
// tidal-error-clang: couldn't infer template argument
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: T appears only before ::type, so no argument gives T a value.
template <class T> struct Id { using type = T; };
template <class T> void only(typename Id<T>::type);
void use() { only(1); }
