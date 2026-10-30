// tidal-rules: LAM-16
// tidal-expect: accept
constexpr auto fact = [](this auto self, int n) -> int {
  return n <= 1 ? 1 : n * self(n - 1);
};
static_assert(fact(5) == 120);
