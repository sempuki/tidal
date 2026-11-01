// tidal-rules: TR-12
// tidal-expect: reject
// tidal-error-clang: expected .\(. after .for.
// tidal-error-gcc: expected .\(. before .constexpr.
// tidal-error-edg: expected a .\(.
// tidal-note: No production puts a keyword between for and the opening parenthesis (Q-0312).
int sum(const int (&values)[3]) {
  int total = 0;
  for constexpr (int v : values) total += v;
  return total;
}
