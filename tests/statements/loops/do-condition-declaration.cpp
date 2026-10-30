// tidal-rules: LOOP-6
// tidal-expect: reject
void f() {
  do {} while (int x = 0);
}
