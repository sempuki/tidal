// tidal-rules: CNS-33
// tidal-expect: reject
// tidal-error: static assertion failed
// tidal-note: The constraint reads a member of Trap<int>, whose definition fails. The error is not a substitution failure, so the other candidate does not help.
template <class T> struct Trap { static_assert(sizeof(T) == 0); static constexpr bool value = true; };
template <class T> requires Trap<T>::value constexpr int f(T) { return 1; }
constexpr int f(...) { return 2; }
static_assert(f(1) == 2);
