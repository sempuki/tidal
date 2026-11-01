// tidal-rules: CNS-1
// tidal-expect: accept
// tidal-note: Checking Caught<int> instantiates Trap<int>, whose static assertion fails. Each call is valid only if the first constraint in the order fails first and stops the check.
template <class T> struct Trap {
  static_assert(sizeof(T) == 0, "instantiated");
  static constexpr bool value = true;
};
template <class T> concept Big = sizeof(T) > 100;
template <class T> concept Caught = Trap<T>::value;
template <Big T> requires Caught<T> constexpr int head(T) { return 1; }
constexpr int head(...) { return 2; }
template <class T> constexpr int tail(Big auto, T) requires Caught<T> { return 1; }
constexpr int tail(...) { return 2; }
static_assert(head(1) == 2 && tail(1, 1) == 2);
