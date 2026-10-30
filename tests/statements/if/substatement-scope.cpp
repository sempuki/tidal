// tidal-rules: IF-8
// tidal-expect: reject
int f() {
  if (true) int y = 1;
  return y;
}
