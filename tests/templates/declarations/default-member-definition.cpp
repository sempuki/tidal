// tidal-rules: TPL-17
// tidal-expect: reject
// tidal-error-clang: cannot add a default template argument to the definition of
// tidal-error-gcc: default argument for template parameter for class enclosing
// tidal-error-edg: a default template argument cannot be specified on the
template <class T> struct Box {
  template <class U> U get();
};
template <class T> template <class U = int> U Box<T>::get() { return U(); }
