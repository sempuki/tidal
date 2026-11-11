// tidal-rules: ATTR-19
// tidal-expect: run
int main() {
  int base = 10;
  auto twice = [] [[nodiscard]] (int x) { return 2 * x; };
  auto one = [] [[nodiscard]] { return 1; };
  auto add = [base] [[nodiscard]] [[tidal::note]] (int x) { return x + base; };
  auto same = [] <class T> [[nodiscard]] (T x) { return x; };
  if (twice(3) != 6) return 1;
  if (one() != 1) return 2;
  if (add(5) != 15) return 3;
  if (same(7) != 7) return 4;
  return 0;
}
