// tidal-rules: SW-5
// tidal-expect: run
int f(int v) {
  switch (v) {
    case 1:
      int x;
      x = 5;
      return x;
    case 2:
      x = 7;
      return x;
  }
  return 0;
}
int main() { return f(2) == 7 ? 0 : 1; }
