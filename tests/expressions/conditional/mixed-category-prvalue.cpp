// tidal-rules: COND-3
// tidal-expect: reject
int f(bool c) {
  int x = 1;
  (c ? x : 2) = 3;
  return x;
}
