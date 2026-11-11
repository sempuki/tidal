// tidal-rules: JMP-6 JMP-2
// tidal-expect: run
// tidal-note: X24-2. A continue, a break, or a return before a later initialized declaration in the same body only leaves scopes, so JMP-6 does not apply.
struct Counted {
  int v;
  constexpr Counted(int x) : v(x) {}
  constexpr ~Counted() {}
};
constexpr int even_sum(int limit) {
  int sum = 0;
  for (int i = 0; i < limit; ++i) {
    if (i % 2) continue;
    if (i > 6) break;
    Counted kept{i};
    sum += kept.v;
  }
  return sum;
}
constexpr int first_or_zero(int k) {
  if (k) return 1;
  int stored = k;
  return stored;
}
static_assert(even_sum(10) == 12);
static_assert(first_or_zero(0) == 0 && first_or_zero(5) == 1);
int main() { return even_sum(10) == 12 ? 0 : 1; }
