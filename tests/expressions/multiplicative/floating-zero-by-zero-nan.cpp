// tidal-rules: MUL-11 MUL-20
// tidal-expect: run
// tidal-note: Q-0004. Each type follows IEEE 754 on the x86-64 targets.
volatile float zero_f = 0.0f;
volatile double zero_d = 0.0;
volatile long double zero_l = 0.0L;
int main() {
  float zf = zero_f;          // one read of each volatile object (SEQ-4)
  double zd = zero_d;
  long double zl = zero_l;
  float nan_f = zf / zf;
  double nan_d = zd / zd;
  long double nan_l = zl / zl;
  double again = nan_d / zero_d;
  int bad = 0;
  if (nan_f == nan_f) bad |= 1;
  if (nan_d == nan_d) bad |= 2;
  if (nan_l == nan_l) bad |= 4;
  if (again == again) bad |= 8;
  return bad;
}
