// tidal-rules: ATTR-2
// tidal-expect: reject
void f(int k) {
  [[fallthrough]];
  (void)k;
}
