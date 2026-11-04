// tidal-rules: CONV-11
// tidal-expect: run
// tidal-note: Q-0028. The conversion happens at run time. The result is the largest finite float or infinity, and both are above 3e38.
int main() {
  volatile double big = 1e300;
  float f = big;
  return f > 3.0e38f ? 0 : 1;
}
