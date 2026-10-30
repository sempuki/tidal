// tidal-rules: LOOP-4
// tidal-expect: reject
int f(int n) {
  while (n-- > 0) int x = n;
  return x;
}
