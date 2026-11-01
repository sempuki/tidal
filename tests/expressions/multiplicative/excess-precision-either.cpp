// tidal-rules: MUL-17
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third). Rounding each operation to double gives the second value; extra precision gives the first. No step overflows (MUL-12).
volatile double tenth = 0.1;
int main() {
  double x = tenth * 3.0 / 3.0;
  return (x == 0.1 || x == 0.10000000000000002) ? 0 : 1;
}
