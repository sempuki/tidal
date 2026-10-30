// tidal-rules: SW-1
// tidal-expect: run
int main() {
  int r = 0;
  for (int i = 0; i < 3; ++i) {
    switch (i) { case 1: break; default: r += 1; }
  }
  return r == 2 ? 0 : 1;
}
