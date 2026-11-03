// tidal-rules: TR-13
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: In instantiation of .constexpr const int Odd<101>::value.
// tidal-error-edg: excessive recursion at instantiation
// tidal-note: Q-0351. The chain is 1000 deep, 500 instantiations of each template. Clang allows 1024 nested instantiations, GCC 900, and EDG 64 of one template.
template <int N> struct Odd;
template <int N> struct Even { static constexpr int value = Odd<N - 1>::value + 1; };
template <int N> struct Odd { static constexpr int value = Even<N - 1>::value + 1; };
template <> struct Even<0> { static constexpr int value = 0; };
static_assert(Even<1000>::value == 1000);
