// tidal-rules: CNS-4 CNS-7
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-expect-clang-18: reject
// tidal-error: static assertion failed
// tidal-note: The mapping of sizeof(T) > 1 holds only T, so the argument T::missing is never substituted. EDG and Clang 18 substitute it and find the concept not satisfied (Q-0284).
template <class T, class U> concept First = sizeof(T) > 1;
template <class T> concept Outer = First<T, typename T::missing>;
static_assert(Outer<int>);
