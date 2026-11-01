// tidal-rules: CNS-2 CNS-23
// tidal-expect: accept
// tidal-note: The parentheses do not make one atomic constraint, so the first clause is a conjunction that contains A<T>.
template <class T> concept A = sizeof(T) > 1;
template <class T> struct Box {
  static constexpr int get() requires (A<T> && (sizeof(T) < 100)) { return 1; }
  static constexpr int get() requires A<T> { return 2; }
};
static_assert(Box<int>::get() == 1);
