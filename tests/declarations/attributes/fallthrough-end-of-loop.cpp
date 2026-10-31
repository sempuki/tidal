// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
int f(int k) {
  int n = 0;
  switch (k) {
  case 0:
    for (int i = 0; i < 2; ++i) { ++n; [[fallthrough]]; }
  case 1:
    ++n;
  }
  return n;
}
