// tidal-rules: SHF-7
// tidal-expect: run-fail
// tidal-flags: -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: shift exponent 32 is too large
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  volatile int count = 31 + argc;  // the width of int
  return 1 << count;
}
