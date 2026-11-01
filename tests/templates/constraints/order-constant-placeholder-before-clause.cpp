// tidal-rules: CNS-1
// tidal-expect: accept
// tidal-note: The constraint of N belongs to the template head, so it is checked before the requires clause. It fails for 'a', and Trap<int> is never instantiated.
template <class T> concept Wide = sizeof(T) > 1;
template <class T> struct Trap { static_assert(sizeof(T) == 0); static constexpr bool value = true; };
template <class T> concept Trapped = Trap<T>::value;
template <class T, Wide auto N> requires Trapped<T> constexpr int f() { return 1; }
template <class T, auto N> constexpr int f() { return 2; }
static_assert(f<int, 'a'>() == 2);
