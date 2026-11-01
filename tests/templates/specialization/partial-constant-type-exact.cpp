// tidal-rules: TSP-14 DED-12
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: static assertion failed
// tidal-note: The partial specialization's parameter V has type int. The argument 1L has type long, so the partial specialization does not match it. Clang converts 1L and 'c' to int and matches (Q-0295).
template <auto V> struct Kind { static constexpr int value = 0; };
template <int V> struct Kind<V> { static constexpr int value = 1; };
static_assert(Kind<1>::value == 1);
static_assert(Kind<1L>::value == 0);
static_assert(Kind<'c'>::value == 0);
