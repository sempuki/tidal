// tidal-rules: TPL-43
// tidal-expect: reject
// tidal-error-clang: unexpanded parameter pack
// tidal-error-gcc: parameter packs not expanded
// tidal-error-edg: parameter pack "T" was referenced but not expanded
template <class... T> struct List { int count(); };
template <class... T> int List<T>::count() { return 0; }
