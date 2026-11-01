// tidal-rules: CNS-1
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-note: The leading clause Big<T> fails for int, so the constraint of the placeholder parameter is not checked. Clang checks the placeholder's constraint first and instantiates Trap<int> (Q-0262).
template <class T> struct Trap {
  static_assert(sizeof(T) == 0, "instantiated");
  static constexpr bool value = true;
};
template <class T> concept Big = sizeof(T) > 100;
template <class T> concept Caught = Trap<T>::value;
template <class T> requires Big<T> constexpr int pick(T, Caught auto) { return 1; }
constexpr int pick(...) { return 2; }
static_assert(pick(1, 1) == 2);
