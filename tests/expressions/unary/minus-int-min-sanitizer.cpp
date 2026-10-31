// tidal-rules: UN-4
// tidal-expect: run-fail
// tidal-flags: -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: negation of -2147483648 cannot be represented
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  volatile int lowest = -2147483647 - argc;
  int negated = -lowest;
  return negated == 0;
}
