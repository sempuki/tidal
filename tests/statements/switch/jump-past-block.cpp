// tidal-rules: SW-5
// tidal-expect: accept
void f(int v) {
  switch (v) {
    case 1: { int x = 1; (void)x; }
    case 2: break;
  }
}
