// tidal-rules: CONV-3
// tidal-expect: run
// tidal-note: Gap review 2026-10-30. The NaN comes from 0.0 / 0.0 in double, which follows IEEE 754 on the matrix targets (MUL-11, MUL-20, Q-0004).
volatile double zero = 0.0;
int main() {
  double z = zero;   // one read of the volatile object (SEQ-4)
  double nan = z / z;
  bool b = nan;
  return b ? 0 : 1;
}
