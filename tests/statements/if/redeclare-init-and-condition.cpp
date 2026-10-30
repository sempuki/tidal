// tidal-rules: IF-10
// tidal-expect: reject
void f() {
  if (int x = 1; int x = 2) {}
}
