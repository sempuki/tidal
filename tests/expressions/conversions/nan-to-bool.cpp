// tidal-rules: CONV-3
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
volatile double zero = 0.0;
int main() {
  double nan = zero / zero;
  bool b = nan;
  return b ? 0 : 1;
}
