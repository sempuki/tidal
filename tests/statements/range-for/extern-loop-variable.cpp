// tidal-rules: RFOR-5
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
void f() {
  int a[2] = {1, 2};
  for (extern int v : a) (void)v;
}
