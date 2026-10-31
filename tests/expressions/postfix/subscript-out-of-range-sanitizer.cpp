// tidal-rules: PFX-4
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: index 4 out of bounds
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  int cells[4] = {};
  volatile int i = 3 + argc;
  return cells[i];
}
