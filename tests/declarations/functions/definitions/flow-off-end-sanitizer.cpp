// tidal-rules: FN-3
// tidal-expect: run-fail
// tidal-flags: -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: execution reached the end of a value-returning function
// tidal-note: Run-time check with the sanitizer (decision 0028).
int pick(int n) {
  if (n > 5) return 1;
}
int main(int argc, char**) { return pick(argc); }
