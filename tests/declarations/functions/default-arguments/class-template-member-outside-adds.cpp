// tidal-rules: DEFARG-13
// tidal-expect: reject
// tidal-error-clang: default arguments cannot be added to an out-of-line definition of a member of a class template
// tidal-error-gcc: may not have default arguments
// tidal-error-edg: default argument is not allowed
// tidal-note: A member function of a class template takes its default arguments only from its declaration in the class. The definition outside the class must not add one, also if no call uses it.
template <class T> struct Shelf { void put(int); };
template <class T> void Shelf<T>::put(int = 1) {}
