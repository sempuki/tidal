// tidal-rules: IF-7
// tidal-expect: accept
void f() {
  if (using T = long; T v = 1) { T w = v; (void)w; }
}
