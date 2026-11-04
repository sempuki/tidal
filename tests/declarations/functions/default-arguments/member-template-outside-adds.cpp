// tidal-rules: DEFARG-13
// tidal-expect: reject
// tidal-error-clang: default arguments cannot be added to a function template
// tidal-error-gcc: may not have default arguments
// tidal-error-edg: default argument is not allowed
// tidal-note: A member function template of a plain class is a function template, so its definition outside the class must not add a default argument.
struct Shelf { template <class T> void put(T, int); };
template <class T> void Shelf::put(T, int = 1) {}
