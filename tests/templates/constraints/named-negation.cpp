// tidal-rules: CNS-4 CNS-23
// tidal-expect: accept
// tidal-note: The negation is written once, inside NotA, so both clauses use the same atomic constraint.
template <class T> concept A = sizeof(T) > 1;
template <class T> concept NotA = !A<T>;
template <class T> struct Box {
  static constexpr int f() requires NotA<T> { return 1; }
  static constexpr int f() requires NotA<T> && (sizeof(T) > 0) { return 2; }
};
static_assert(Box<char>::f() == 2);
