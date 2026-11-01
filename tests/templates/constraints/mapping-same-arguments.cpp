// tidal-rules: CNS-4
// tidal-expect: accept
// tidal-note: Both wrappers map N of Inner to sizeof(T) + 1, so they give the same atomic constraint, and the second template is more constrained.
template <int N> concept Inner = N > 2;
template <class T> concept First = Inner<sizeof(T) + 1>;
template <class T> concept Second = Inner<sizeof(T) + 1>;
template <class T> requires First<T> constexpr int f(T) { return 1; }
template <class T> requires Second<T> && true constexpr int f(T) { return 2; }
static_assert(f(1) == 2);
