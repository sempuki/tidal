// tidal-rules: IF-10
// tidal-expect: accept
void f() {
  if (int x = 1) {
    { int x = 2; (void)x; }
  }
}
