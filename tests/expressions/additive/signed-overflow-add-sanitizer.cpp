// tidal-rules: ADD-9
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: signed integer overflow
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  volatile int top = 2147483647;
  int sum = top + argc;  // argc is 1, so the sum does not fit
  return sum < 0 ? 0 : 1;
}
