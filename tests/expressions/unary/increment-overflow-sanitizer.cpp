// tidal-rules: UN-21
// tidal-expect: run-fail
// tidal-flags: -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: signed integer overflow
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  volatile int top = 2147483647;
  int n = top;
  ++n;
  return n == argc;
}
