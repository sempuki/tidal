// tidal-rules: LOOP-4
// tidal-expect: reject
int f() {
  for (int i = 0; i < 2; ++i) {}
  return i;
}
