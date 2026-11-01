// tidal-rules: TSP-26
// tidal-expect: reject
// tidal-error-clang: does not refer into a class, class template or class template partial specialization
// tidal-error-gcc: invalid explicit specialization
// tidal-error-edg: a template declaration containing a template parameter list
// tidal-note: Inner<int> is fixed while Outer keeps its parameter T.
template <class T> struct Outer {
  template <class U> struct Inner { int get(); };
};
template <class T> template <> int Outer<T>::Inner<int>::get() { return 1; }
