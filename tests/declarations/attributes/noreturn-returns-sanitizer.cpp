// tidal-rules: ATTR-3
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: unreachable program point
// tidal-error-gcc-14: exit 139
// tidal-note: GCC 14 does not report it, and the program crashes.
// tidal-note: Run-time check with the sanitizer (decision 0028).
[[noreturn]] void stop(int n) {
  if (n > 5) throw n;
}
int main(int argc, char**) { stop(argc); }
