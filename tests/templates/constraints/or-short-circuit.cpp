// tidal-rules: CNS-6
// tidal-expect: accept
// tidal-note: The left operands are true, so Trap<int> is not instantiated and Count::value, an int, is not checked.
template <class T> struct Trap {
  static_assert(sizeof(T) == 0, "instantiated");
  static constexpr bool value = true;
};
struct Count { static constexpr int value = 1; };
template <class T> concept Either = (sizeof(T) > 0) || Trap<T>::value;
template <class T> concept Valued = (sizeof(T) > 0) || T::value;
static_assert(Either<int> && Valued<Count>);
