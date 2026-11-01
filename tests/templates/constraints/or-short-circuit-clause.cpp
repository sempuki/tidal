// tidal-rules: CNS-6
// tidal-expect: accept
// tidal-note: The left operand of each clause is satisfied, so Trap<T> is not instantiated. When the left operand is not satisfied, the right operand decides.
template <class T> struct Trap {
  static_assert(sizeof(T) == 0, "instantiated");
  static constexpr bool value = true;
};
template <class T> requires (sizeof(T) > 0) || Trap<T>::value constexpr int any(T) { return 1; }
template <class T> concept Either = (sizeof(T) > 100) || (sizeof(T) == 1);
static_assert(any(5) == 1 && any('c') == 1);
static_assert(Either<char> && !Either<int>);
