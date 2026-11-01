// tidal-rules: OVL-20
// tidal-expect: accept
// tidal-expect-gcc-14: reject
// tidal-expect-edg: reject
// tidal-error-gcc-14: unable to deduce .const auto. from
// tidal-error-edg: cannot deduce "auto" type
// tidal-note: Q-0160. With no target type, the constraint of Mode<2>::f() requires (N == 1) is not satisfied, so that function leaves the set and one function remains. GCC 14 and EDG find no single function.
template <int N> struct Mode {
  static constexpr int f() requires (N == 1) { return 1; }
  static constexpr int f() requires (N != 1) { return 2; }
};
constexpr auto one = &Mode<1>::f;
constexpr auto two = &Mode<2>::f;
static_assert(one() == 1 && two() == 2);
