// tidal-rules: IF-9
// tidal-expect: accept
void f() {
  if (true) {
  } else if (int x = 1) {
  } else {
    (void)x;
  }
}
