// tidal-rules: PRIM-27
// tidal-expect: accept
// tidal-note: In a left fold the extra operand is the innermost left operand, (100 - 10) - 1. In a right fold it is the innermost right operand, 10 - (1 - 100).
template <int... N> constexpr int left = (100 - ... - N);
template <int... N> constexpr int right = (N - ... - 100);
static_assert(left<10, 1> == 89);
static_assert(right<10, 1> == 109);
