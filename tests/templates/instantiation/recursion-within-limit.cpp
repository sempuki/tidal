// tidal-rules: TR-13
// tidal-expect: accept
template <int N> struct Count { static constexpr int value = Count<N - 1>::value + 1; };
template <> struct Count<0> { static constexpr int value = 0; };
static_assert(Count<50>::value == 50);
