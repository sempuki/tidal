// tidal-rules: MUL-11
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=float-divide-by-zero -fno-sanitize-recover=all
// tidal-error: division by zero
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  volatile double zero = argc - 1;
  double ratio = 1.0 / zero;
  return ratio > 0;
}
