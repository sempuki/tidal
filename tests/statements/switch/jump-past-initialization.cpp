// tidal-rules: SW-5
// tidal-expect: reject
void f(int v) {
  switch (v) {
    case 1:
      int x = 1;
    case 2:
      break;
  }
}
