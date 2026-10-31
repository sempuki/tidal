// tidal-rules: ATTR-10
// tidal-expect: run-fail
// tidal-flags: -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error-clang: assumption is violated
// tidal-error-gcc: unreachable program point
// tidal-expect-clang-18: run
// tidal-note: Clang 18 ignores the attribute.
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  [[assume(argc > 5)]];
  return 0;
}
