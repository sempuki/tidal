// tidal-rules: SW-3
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int pick(char c) {
  switch (c) {
    case 300: return 1;
    default: return 2;
  }
}
int main() { return pick(44) == 2 ? 0 : 1; }
