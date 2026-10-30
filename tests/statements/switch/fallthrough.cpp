// tidal-rules: SW-1
// tidal-expect: run
int count(int v) {
  int r = 0;
  switch (v) {
    case 0: r += 1; [[fallthrough]];
    case 1: r += 10; break;
    case 2: r += 100;
  }
  return r;
}
int main() { return count(0) == 11 && count(1) == 10 && count(2) == 100 ? 0 : 1; }
