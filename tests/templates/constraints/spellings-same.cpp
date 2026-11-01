// tidal-rules: CNS-1 CNS-23
// tidal-expect: accept
// tidal-note: A concept before the parameter, in the leading clause, and in the trailing clause gives one associated constraint, so AB subsumes A in each pair.
template <class T> concept A = sizeof(T) > 1;
template <class T> concept AB = A<T> && sizeof(T) < 100;
template <A T> constexpr int f(T) { return 1; }
template <class T> requires AB<T> constexpr int f(T) { return 2; }
template <class T> constexpr int g(T) requires A<T> { return 1; }
template <AB T> constexpr int g(T) { return 2; }
template <class T> requires A<T> constexpr int h(T) { return 1; }
template <class T> constexpr int h(T) requires AB<T> { return 2; }
static_assert(f(1) == 2 && g(1) == 2 && h(1) == 2);
