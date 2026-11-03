// tidal-rules: COND-10
// tidal-expect: accept
// tidal-note: The second operand is a comma expression of two assignments. Both run when the condition is true.
constexpr int f(bool c) {
  int x = 0, y = 0;
  c ? x = 1, y = 2 : y = 3;
  return x * 10 + y;
}
static_assert(f(true) == 12);
static_assert(f(false) == 3);
