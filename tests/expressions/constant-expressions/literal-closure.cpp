// tidal-rules: CE-23
// tidal-expect: accept
constexpr auto one = [] { return 1; };
static_assert(one() == 1);
constexpr int add_three(int base) {
  auto add = [base](int v) { return base + v; };
  return add(3);
}
static_assert(add_three(2) == 5);
