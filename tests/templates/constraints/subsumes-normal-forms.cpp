// tidal-rules: CNS-22
// tidal-expect: accept
// tidal-note: In f, each clause of the second disjunction contains A or B. In g, the conjunction contains A, which subsumes the disjunction of A and C.
template <class T> concept A = sizeof(T) > 1;
template <class T> concept B = sizeof(T) < 100;
template <class T> concept C = alignof(T) > 0;
template <class T> struct Box {
  static constexpr int f() requires A<T> || B<T> { return 1; }
  static constexpr int f() requires (A<T> && C<T>) || (B<T> && C<T>) { return 2; }
  static constexpr int g() requires A<T> || C<T> { return 1; }
  static constexpr int g() requires A<T> && B<T> { return 2; }
};
static_assert(Box<int>::f() == 2 && Box<int>::g() == 2);
