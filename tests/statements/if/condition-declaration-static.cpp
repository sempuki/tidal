// tidal-rules: IF-6
// tidal-expect: reject
void f() {
  if (static int x = 1) {}
}
