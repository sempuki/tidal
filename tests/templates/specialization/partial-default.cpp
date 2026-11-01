// tidal-rules: TSP-11
// tidal-expect: reject
// tidal-error-clang: default template argument in a class template partial
// tidal-error-gcc: default template arguments may not be used in partial
// tidal-error-edg: partial specializations may not have default template
template <class T, class U> struct Pair {};
template <class T, class U = int> struct Pair<T*, U> {};
