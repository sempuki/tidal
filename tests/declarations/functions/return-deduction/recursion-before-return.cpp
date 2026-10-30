// tidal-rules: RET-3
// tidal-expect: reject
auto f(int n) {
  if (n > 0) return f(n - 1);
  return 0;
}
