// tidal-rules: DEP-39
// tidal-expect: accept
// tidal-note: No class has a member nothing, and the template is never instantiated. The definition checks no member name after a type-dependent object.
template <class T> int call(T t) { return t.nothing(); }
