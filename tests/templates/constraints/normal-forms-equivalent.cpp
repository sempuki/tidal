// tidal-rules: CNS-22 CNS-24
// tidal-expect: reject
// tidal-error-clang: is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: The two clauses have the same normal forms, so each subsumes the other, and neither is more constrained.
template <class T> concept A = sizeof(T) > 1;
template <class T> concept B = sizeof(T) < 100;
template <class T> concept C = alignof(T) > 0;
template <class T> struct Box {
  static constexpr int f() requires (A<T> || B<T>) && C<T> { return 1; }
  static constexpr int f() requires (A<T> && C<T>) || (B<T> && C<T>) { return 2; }
};
int x = Box<int>::f();
