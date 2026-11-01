// tidal-rules: CNS-4 CNS-24
// tidal-expect: reject
// tidal-error-clang: is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: A and A2 have the same text, but their atomic constraints come from two expressions.
template <class T> concept A = sizeof(T) > 1;
template <class T> concept A2 = sizeof(T) > 1;
template <class T> struct Box {
  static constexpr int f() requires A<T> { return 1; }
  static constexpr int f() requires A2<T> && true { return 2; }
};
int x = Box<int>::f();
