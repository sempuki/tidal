// tidal-rules: MUL-10
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: signed integer overflow
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  volatile int half = 65536;
  int h = half;               // one read of the volatile object (SEQ-4)
  int product = h * (h + argc);
  return product == 0;
}
