// tidal-rules: MUL-9
// tidal-expect: run-fail
// tidal-flags: -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: division by zero
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  volatile int zero = argc - 1;
  return 10 / zero;
}
