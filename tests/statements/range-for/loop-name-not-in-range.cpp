// tidal-rules: RFOR-8
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
int items[3] = {1, 2, 3};
int main() { int sum = 0; for (int items : items) sum += items; return sum == 6 ? 0 : 1; }
