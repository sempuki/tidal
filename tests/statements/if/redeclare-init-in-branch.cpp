// tidal-rules: IF-10
// tidal-expect: reject
void f() {
  if (int x = 1; x) {
    int x = 2;
  }
}
