// tidal-rules: RFOR-6
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
void f() {
  int a[2] = {1, 2};
  for (int v : a) {
    int v = 3;
  }
}
