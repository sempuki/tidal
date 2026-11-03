// tidal-rules: SHF-8
// tidal-expect: accept
// tidal-expect-gcc-host: reject
// tidal-expect-gcc-16: reject
// tidal-error-gcc: static assertion failed
// tidal-flags: -O0 -Wall -fsanitize=shift -fno-sanitize-recover=shift
// tidal-note: Q-0332. Write report W2. GCC 16 with the shift sanitizer and -Wall calls the left operand twice during constant evaluation, so the count is 3.
struct Log {
  int seq[4]{};
  int n = 0;
  constexpr int mark(int id) { seq[n++] = id; return id; }
};
constexpr int calls() {
  Log log;
  int r = log.mark(1) << log.mark(3);
  return r == 8 ? log.n : -1;
}
static_assert(calls() == 2);
