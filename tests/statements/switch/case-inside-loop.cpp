// tidal-rules: SW-1
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int run(int k) {
  int n = 0;
  switch (k) {
    case 0:
      while (n < 3) {
        n += 10;
    case 1:
        n += 1;
      }
  }
  return n;
}
int main() { return run(1) == 12 && run(0) == 11 ? 0 : 1; }
