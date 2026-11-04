// tidal-rules: CONV-7
// tidal-expect: run
// tidal-note: The conversions happen at run time. 33554433 lies nearer to 33554432, and 16777219 and 1 + 2^-24 lie halfway, so each goes to the neighbor whose last bit is zero.
volatile int near_source = 33554433;
volatile int tie_source = 16777219;
volatile double tie_down_source = 1.0 + 0x1p-24;
volatile double above_source = 1.0 + 0x1.8p-24;
int main() {
  volatile float near = near_source;
  volatile float tie = tie_source;
  volatile float tie_down = tie_down_source;
  volatile float above = above_source;
  if (near != 33554432.0f) return 1;
  if (tie != 16777220.0f) return 2;
  if (tie_down != 1.0f) return 3;
  if (above != 1.0f + 0x1p-23f) return 4;
  return 0;
}
