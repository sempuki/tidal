// tidal-rules: CONV-4
// tidal-expect: run
// tidal-note: The conversions happen at run time, from double and from float.
int main() {
  volatile double a = -0.5;
  volatile double b = 1000000000.75;
  volatile float c = 7.9f;
  int ia = a;
  long ib = b;
  unsigned ic = c;
  return ia == 0 && ib == 1000000000 && ic == 7 ? 0 : 1;
}
