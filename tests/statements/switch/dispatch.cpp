// tidal-rules: SW-1
// tidal-expect: run
int pick(int v) {
  int r = 0;
  switch (v) {
    case 1: r = 10; break;
    case 2: r = 20; break;
    default: r = -1; break;
  }
  return r;
}
int main() { return pick(1) == 10 && pick(2) == 20 && pick(3) == -1 ? 0 : 1; }
