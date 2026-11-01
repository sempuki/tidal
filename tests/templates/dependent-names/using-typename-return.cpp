// tidal-rules: DEP-23
// tidal-expect: accept
// tidal-note: The using-declaration with typename makes size_type a type name in Derived, which then names the return type and a parameter type.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Base { using size_type = unsigned; };
template <class T> struct Derived : Base<T> {
  using typename Base<T>::size_type;
  static constexpr size_type twice(size_type n) { return n * 2; }
};
static_assert(same<decltype(Derived<char>::twice(2)), unsigned>);
static_assert(Derived<char>::twice(2) == 4);
