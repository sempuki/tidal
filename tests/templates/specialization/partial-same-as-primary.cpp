// tidal-rules: TSP-11
// tidal-expect: reject
// tidal-error-clang: class template partial specialization does not specialize
// tidal-error-gcc: does not specialize any template arguments and is not more
// tidal-error-edg: a template argument list is not allowed in a declaration of
template <class T, class U> struct Pair {};
template <class T, class U> struct Pair<T, U> {};
