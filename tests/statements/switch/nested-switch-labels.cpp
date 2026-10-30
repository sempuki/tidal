// tidal-rules: SW-1
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int pick(int a, int b) {
  switch (a) {
    case 1:
      switch (b) {
        case 2: return 12;
      }
      return 10;
    default: return 0;
  }
}
int main() { return pick(2, 2) == 0 && pick(1, 2) == 12 && pick(1, 3) == 10 ? 0 : 1; }
