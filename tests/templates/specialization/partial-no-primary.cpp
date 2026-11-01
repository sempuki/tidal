// tidal-rules: TSP-31
// tidal-expect: reject
// tidal-error-clang: explicit specialization of undeclared template struct
// tidal-error-gcc: is not a class template
// tidal-error-edg: a template argument list is not allowed
template <class T> struct Box<T*> {};
