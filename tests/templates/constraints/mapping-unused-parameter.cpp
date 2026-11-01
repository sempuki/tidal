// tidal-rules: CNS-4
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: The atom sizeof(T) > 1 does not use U, so First<T, int> and First<T, long> give the same atomic constraint, and the second template is more constrained. EDG finds the call ambiguous (Q-0284).
template <class T, class U> concept First = sizeof(T) > 1;
template <class T> requires First<T, int> constexpr int f(T) { return 1; }
template <class T> requires First<T, long> && true constexpr int f(T) { return 2; }
static_assert(f(1) == 2);
