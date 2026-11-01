// tidal-rules: TYP-14
// tidal-expect: run
volatile double one = 1.0;
volatile double half_ulp = 0x1p-53;
volatile double three_half_ulp = 0x1.8p-52;
volatile double above_half_ulp = 0x1.1p-53;
volatile float one_f = 1.0f;
volatile float half_ulp_f = 0x1p-24f;
int main() {
  double tie_down = one + half_ulp;
  double tie_up = one + three_half_ulp;
  double above = one + above_half_ulp;
  float tie_f = one_f + half_ulp_f;
  if (tie_down != 1.0) return 1;
  if (tie_up != 1.0 + 0x1p-51) return 2;
  if (above != 1.0 + 0x1p-52) return 3;
  if (tie_f != 1.0f) return 4;
  return 0;
}
