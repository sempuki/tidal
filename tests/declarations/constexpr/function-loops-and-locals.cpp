// tidal-rules: CX-3
// tidal-expect: accept
constexpr int sum(int n) {
  int total = 0;
  for (int i = 1; i <= n; ++i) total += i;
  return total;
}
static_assert(sum(4) == 10);
