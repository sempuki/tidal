// tidal-rules: CNS-30
// tidal-expect: accept
// tidal-note: The type of a specialization of a constrained function template has no trace of the requires clause.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> int pick(T) requires (sizeof(T) > 1) { return 1; }
static_assert(same<decltype(pick<int>), int(int)>);
int (*p)(int) = pick<int>;
