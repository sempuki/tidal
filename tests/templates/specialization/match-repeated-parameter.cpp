// tidal-rules: TSP-14
// tidal-expect: accept
// tidal-note: T appears twice, so both arguments must be the same type.
template <class T, class U> struct Same { static constexpr bool value = false; };
template <class T> struct Same<T, T> { static constexpr bool value = true; };
static_assert(Same<int, int>::value);
static_assert(!Same<int, long>::value);
static_assert(!Same<int, const int>::value);
