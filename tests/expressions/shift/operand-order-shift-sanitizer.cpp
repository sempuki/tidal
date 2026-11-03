// tidal-rules: SHF-8
// tidal-expect: run
// tidal-expect-gcc-host: run-fail
// tidal-expect-gcc-16: run-fail
// tidal-error-gcc: exit 1
// tidal-flags: -O0 -Wall -fsanitize=shift -fno-sanitize-recover=shift
// tidal-note: Q-0332. Write report W2. GCC 16 with the shift sanitizer and -Wall calls the constexpr function in the left operand twice.
struct Log {
  int count = 0;
  constexpr int mark(int v) { ++count; return v; }
};
int main() {
  Log log;
  int r = log.mark(1) << log.mark(3);
  return r == 8 && log.count == 2 ? 0 : 1;
}
