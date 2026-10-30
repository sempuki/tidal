// tidal-rules: RET-3
// tidal-expect: accept
constexpr auto fact(int n) {
  if (n <= 1) return 1;
  return n * fact(n - 1);
}
static_assert(fact(5) == 120);
