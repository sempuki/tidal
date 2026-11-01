// tidal-rules: TSP-17
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: after instantiation of
// tidal-error-edg: this partial specialization would have been used to
// tidal-note: Kind<int*> is complete before the partial specialization that would match it appears. Clang does not diagnose the order, which TSP-17 allows (Q-0173).
template <class T> struct Kind { static constexpr int value = 0; };
Kind<int*> early;
template <class T> struct Kind<T*> { static constexpr int value = 1; };
