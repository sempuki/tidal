// tidal-rules: PACK-10
// tidal-expect: accept
// tidal-note: A fixed argument and the expansion N... together give the three constant parameters of Three, which are not packs.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <int A, int B, int C> struct Three { static constexpr int sum = A + B + C; };
template <int... N> struct Hold { using type = Three<0, N...>; };
static_assert(same<Hold<1, 2>::type, Three<0, 1, 2>>);
static_assert(Hold<4, 5>::type::sum == 9);
