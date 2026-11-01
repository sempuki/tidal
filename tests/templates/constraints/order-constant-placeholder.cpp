// tidal-rules: CNS-1
// tidal-expect: accept
// tidal-note: The constraint of N comes first and fails for 'a', so Trapped<int> is never checked and Trap<int> is never instantiated.
template <class T> concept Wide = sizeof(T) > 1;
template <class T> struct Trap { static_assert(sizeof(T) == 0); static constexpr bool value = true; };
template <class T> concept Trapped = Trap<T>::value;
template <Wide auto N, Trapped T> constexpr int f(T) { return 1; }
template <auto N, class T> constexpr int f(T) { return 2; }
static_assert(f<'a'>(1) == 2);
