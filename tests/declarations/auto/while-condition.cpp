// tidal-rules: AUTO-12
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int main() { int n = 3; int steps = 0; while (auto left = n--) steps += left; return steps == 6 ? 0 : 1; }
