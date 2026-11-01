// tidal-rules: MUL-11 MUL-20
// tidal-expect: run
// tidal-note: Q-0004. Each type follows IEEE 754 on the x86-64 targets.
volatile float zero_f = 0.0f;
volatile double zero_d = 0.0;
volatile long double zero_l = 0.0L;
volatile double negative_zero = -0.0;
int main() {
  float inf_f = 1.0f / zero_f;
  double inf_d = 1.0 / zero_d;
  long double inf_l = 1.0L / zero_l;
  double minus_d = -1.0 / zero_d;
  double flipped = 1.0 / negative_zero;
  double both = -1.0 / negative_zero;
  int bad = 0;
  if (!(inf_f > 3.4e38f) || inf_f != inf_f * 2.0f) bad |= 1;
  if (!(inf_d > 1.7e308) || inf_d != inf_d * 2.0) bad |= 2;
  if (!(inf_l > 1.1e4932L) || inf_l != inf_l * 2.0L) bad |= 4;
  if (!(minus_d < -1.7e308) || minus_d != -inf_d) bad |= 8;
  if (flipped != minus_d) bad |= 16;
  if (both != inf_d) bad |= 32;
  if (inf_d / zero_d != inf_d) bad |= 64;
  return bad;
}
