// tidal-rules: TR-13
// tidal-expect: accept
// tidal-note: The chain is 100 deep and alternates between two templates, 50 instantiations of each. EDG counts the instantiations of each template on its own, so it accepts this chain.
template <int N> struct Odd;
template <int N> struct Even { static constexpr int value = Odd<N - 1>::value + 1; };
template <int N> struct Odd { static constexpr int value = Even<N - 1>::value + 1; };
template <> struct Even<0> { static constexpr int value = 0; };
static_assert(Even<100>::value == 100);
