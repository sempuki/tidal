// tidal-rules: IF-5
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
int main() { int r = 0; if (int x = {3}) r = x; while (int y = {0}) r = 9; return r == 3 ? 0 : 1; }
