// tidal-rules: IF-6
// tidal-expect: reject
void f() {
  if (extern int x = 1) {}
}
