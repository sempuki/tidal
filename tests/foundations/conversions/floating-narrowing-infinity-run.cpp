// tidal-rules: CONV-11
// tidal-expect: run
// tidal-note: The conversion happens at run time, and Clang and GCC give infinity. Only an infinity equals twice its value.
volatile double big = 1e300;
int main() {
  volatile float f = big;
  volatile float twice = f * 2.0f;
  return f > 0 && f == twice ? 0 : 1;
}
