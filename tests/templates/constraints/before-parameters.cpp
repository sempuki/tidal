// tidal-rules: CNS-11
// tidal-expect: accept
// tidal-note: The constraint fails, so the parameter of type Trap<int> is not formed and the conversion of 0 to it is not checked.
template <class T> struct Trap { static_assert(sizeof(T) == 0); };
template <class T> requires (sizeof(T) > 100) int take(T, Trap<T>) { return 1; }
constexpr int take(...) { return 2; }
static_assert(take(1, 0) == 2);
