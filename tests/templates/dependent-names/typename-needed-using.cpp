// tidal-rules: DEP-19
// tidal-expect: reject
// tidal-error-clang: unknown type name 'type'
// tidal-error-gcc: .type. does not name a type
// tidal-error-edg: is not a type name
template <class T> struct Base { using type = int; };
template <class T> struct Derived : Base<T> {
  using Base<T>::type;
  type value = 4;
};
Derived<char> d;
