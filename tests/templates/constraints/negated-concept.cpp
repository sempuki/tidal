// tidal-rules: CNS-8
// tidal-expect: accept
// tidal-note: HasValue<int> is false because of the substitution failure inside the concept, so its negation is true.
template <class T> concept HasValue = T::value;
template <class T> requires (!HasValue<T>) constexpr int f(T) { return 1; }
constexpr int f(...) { return 2; }
static_assert(f(1) == 1);
