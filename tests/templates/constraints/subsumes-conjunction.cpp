// tidal-rules: CNS-22 CNS-23
// tidal-expect: accept
template <class T> concept A = sizeof(T) > 1;
template <class T> concept B = sizeof(T) < 100;
template <class T> struct Box {
  static constexpr int f() requires A<T> { return 1; }
  static constexpr int f() requires A<T> && B<T> { return 2; }
  static constexpr int g() requires A<T> || B<T> { return 1; }
  static constexpr int g() requires A<T> { return 2; }
};
static_assert(Box<int>::f() == 2 && Box<int>::g() == 2);
