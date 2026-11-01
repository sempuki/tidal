// tidal-rules: CNS-1 CNS-33
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-expect-gcc-14: accept
// tidal-error: static assertion failed
// tidal-note: The constraints of the head are checked in parameter order, so Trapped<int> comes before the constraint of N and instantiates Trap<int>. EDG and GCC 14 check the constraint of N first (Q-0282).
template <class T> concept Wide = sizeof(T) > 1;
template <class T> struct Trap { static_assert(sizeof(T) == 0); static constexpr bool value = true; };
template <class T> concept Trapped = Trap<T>::value;
template <Trapped T, Wide auto N> constexpr int f() { return 1; }
template <class T, auto N> constexpr int f() { return 2; }
static_assert(f<int, 'a'>() == 2);
