// tidal-rules: LOOP-4
// tidal-expect: reject
void f() {
  for (int i = 0; i < 2; ++i) {
    int i = 5;
  }
}
