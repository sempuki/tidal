// tidal-rules: TSP-11
// tidal-expect: reject
// tidal-error-clang: class template partial specialization contains a template
// tidal-error-gcc: template parameters not deducible in partial specialization:
// tidal-error-edg: is not used in or cannot be deduced from the template
// tidal-note: U appears nowhere in the argument list, so no argument list gives it a value.
template <class T> struct Box {};
template <class T, class U> struct Box<T*> {};
