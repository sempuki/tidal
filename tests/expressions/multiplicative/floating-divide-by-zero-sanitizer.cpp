// tidal-rules: MUL-11
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-note: Q-0004. The undefined-behavior sanitizer lets the division run, because double follows IEEE 754 here (decision 0028).
int main(int argc, char**) {
  volatile double zero = argc - 1;
  double ratio = 1.0 / zero;
  return ratio > 1.7e308 ? 0 : 1;
}
