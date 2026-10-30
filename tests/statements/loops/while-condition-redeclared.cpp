// tidal-rules: LOOP-4
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
void f() {
  while (int x = 0) {
    int x = 1;
  }
}
