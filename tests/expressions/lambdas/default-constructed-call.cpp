// tidal-rules: LAM-11
// tidal-expect: accept
// tidal-note: A value-initialized closure object of a lambda without captures can be called in a constant expression.
auto twice = [](int x) { return 2 * x; };
using Twice = decltype(twice);
static_assert(Twice{}(4) == 8);
constexpr int use() {
  Twice t;
  t = twice;
  return t(5);
}
static_assert(use() == 10);
