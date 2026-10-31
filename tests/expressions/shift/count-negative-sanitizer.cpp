// tidal-rules: SHF-7
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: shift exponent -1 is negative
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  volatile int count = -argc;
  return 1 << count;
}
