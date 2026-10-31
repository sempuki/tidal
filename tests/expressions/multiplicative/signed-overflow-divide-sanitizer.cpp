// tidal-rules: MUL-10
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: division of -2147483648 by -1 cannot be represented
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  volatile int lowest = -2147483647 - argc;
  volatile int minus_one = -argc;
  return lowest / minus_one == 0;
}
