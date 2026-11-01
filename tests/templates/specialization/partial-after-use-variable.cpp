// tidal-rules: TSP-17
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: after instantiation of
// tidal-note: kind<int*> is used before the partial specialization that would match it appears. GCC diagnoses the order. Clang and EDG do not, which TSP-17 allows (Q-0173).
template <class T> constexpr int kind = 0;
constexpr int early = kind<int*>;
template <class T> constexpr int kind<T*> = 1;
