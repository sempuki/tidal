// tidal-rules: CTAD-6 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Only" matches
// tidal-note: The class template is declared and not defined, so its one guide has no parameters, and the guide gives T no value.
template <class T> struct Only;
Only d;
