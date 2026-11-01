// tidal-rules: TARG-2
// tidal-expect: accept
// tidal-note: The local variable hides the variable template, so the call has two arguments, v < 1 and 2 > (0). Where v names the template, v<1, 2> is one argument.
template <int A, int B> constexpr int v = A + B;
constexpr int count(bool, bool) { return 2; }
constexpr int count(int) { return 1; }
constexpr int use() {
  int v = 0;
  return count(v < 1, 2 > (0));
}
static_assert(use() == 2);
static_assert(count(v<1, 2>) == 1);
