// tidal-rules: TR-13
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: The limits on the depth of instantiation are the same on both targets. Every compiler allows 50.
template <int N> struct Count { static constexpr int value = Count<N - 1>::value + 1; };
template <> struct Count<0> { static constexpr int value = 0; };
static_assert(Count<50>::value == 50);
