// tidal-rules: TARG-9
// tidal-expect: accept
// tidal-note: The default of M names N and uses its argument. The empty list of zero takes the default type long.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <int N, int M = N * 2> struct Pair { static constexpr int second = M; };
static_assert(Pair<3>::second == 6 && Pair<3, 1>::second == 1);
template <class T = long> constexpr T zero() { return T(); }
static_assert(same<decltype(zero<>()), long>);
