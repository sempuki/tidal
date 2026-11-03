// tidal-rules: MUL-18
// tidal-expect: run
volatile int six = 6, seven = 7;
volatile double one_and_a_half = 1.5, seven_d = 7.0, two = 2.0;
volatile float quarter = 0.25f;
int main() {
  if (six * seven != 42) return 1;
  if (one_and_a_half * two != 3.0) return 2;
  if (seven_d / two != 3.5) return 3;
  float q = quarter;          // one read of the volatile object (SEQ-4)
  if (q * q != 0.0625f) return 4;
  if (-six * seven != -42) return 5;
  return 0;
}
