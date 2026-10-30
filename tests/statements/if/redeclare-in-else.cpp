// tidal-rules: IF-10
// tidal-expect: reject
void f() {
  if (int x = 0) {
  } else {
    int x = 2;
  }
}
