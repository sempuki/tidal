// tidal-rules: CONV-5
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=float-cast-overflow -fno-sanitize-recover=all
// tidal-error: 1e\+10 is outside the range of representable values
// tidal-note: Run-time check with the sanitizer (decision 0028).
int main(int argc, char**) {
  volatile double big = 1e10 * argc;
  int n = big;
  return n == 0;
}
