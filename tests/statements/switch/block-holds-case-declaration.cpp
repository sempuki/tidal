// tidal-rules: SW-5
// tidal-expect: run
int pick(int v) {
  int r = 0;
  switch (v) {
    case 1: {
      int x = 10;
      r = x;
      break;
    }
    case 2:
      r = 20;
      break;
  }
  return r;
}
int main() { return pick(1) == 10 && pick(2) == 20 ? 0 : 1; }
