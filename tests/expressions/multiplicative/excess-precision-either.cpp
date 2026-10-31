// tidal-rules: MUL-17
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
volatile double big = 1e308;
int main() { double x = big * 10.0 / 10.0; return (x == 1e308 || x > 1e308) ? 0 : 1; }
