// tidal-rules: TR-13
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: excessive recursion at instantiation
// tidal-note: Q-0351. Clang and GCC allow 100 nested instantiations. EDG allows 64 instantiations of one template at once.
template <int N> struct Count { static constexpr int value = Count<N - 1>::value + 1; };
template <> struct Count<0> { static constexpr int value = 0; };
static_assert(Count<100>::value == 100);
