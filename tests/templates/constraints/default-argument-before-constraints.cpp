// tidal-rules: CNS-11 CNS-33
// tidal-expect: reject
// tidal-error: static assertion failed
// tidal-note: The default argument of U is formed before the constraint of T is checked, and Trap<char> fails. The constraint would also fail for char.
template <class T> concept Wide = sizeof(T) > 1;
template <class T> struct Trap { static_assert(sizeof(T) == 0); using type = int; };
template <Wide T, class U = typename Trap<T>::type> constexpr int f(T) { return 1; }
constexpr int f(...) { return 2; }
static_assert(f('a') == 2);
