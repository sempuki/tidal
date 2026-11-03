// tidal-rules: PRIM-27
// tidal-expect: accept
// tidal-note: A left and a right fold over <<, which is not associative, and a fold over the comma operator, which runs the copies in the order of the pack.
template <int... V> constexpr int left_shift = (1 << ... << V);
template <int... V> constexpr int right_shift = (V << ... << 1);
static_assert(left_shift<1, 2> == 8);    // (1 << 1) << 2
static_assert(right_shift<1, 2> == 16);  // 1 << (2 << 1)
template <int... V> constexpr int digits() {
  int r = 0;
  ((r = r * 10 + V), ...);
  return r;
}
static_assert(digits<4, 0, 7>() == 407);
