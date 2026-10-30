// tidal-rules: IF-18
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int where() {
  if consteval { return 1; } else { return 2; }
}
int main() { return where() == 2 ? 0 : 1; }
