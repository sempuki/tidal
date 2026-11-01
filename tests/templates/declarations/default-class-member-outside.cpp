// tidal-rules: TPL-17
// tidal-expect: reject
// tidal-error-clang: cannot add a default template argument to the definition of
// tidal-error-gcc: default argument for template parameter for class enclosing
// tidal-error-edg: a default template argument cannot be specified on the
// tidal-note: The definition of a member of a class template outside the class gives a default to the class's own parameter.
template <class T> struct Box { void set(); };
template <class T = int> void Box<T>::set() {}
