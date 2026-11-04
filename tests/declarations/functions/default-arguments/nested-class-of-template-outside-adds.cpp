// tidal-rules: DEFARG-13
// tidal-expect: reject
// tidal-error-clang: default arguments cannot be added to an out-of-line definition of a member of a nested class in a template
// tidal-error-gcc: may not have default arguments
// tidal-error-edg: default argument is not allowed
// tidal-note: A member function of a class nested in a class template is a member of a templated class, so its definition outside the class must not add a default argument.
template <class T> struct Shelf { struct Slot { void put(int); }; };
template <class T> void Shelf<T>::Slot::put(int = 1) {}
