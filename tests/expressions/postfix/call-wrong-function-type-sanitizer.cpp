// tidal-rules: PFX-19
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-expect-gcc: run
// tidal-error-clang: through pointer to incorrect function type
// tidal-note: The GCC sanitizer does not check the type of a called function.
int seen = 0;
void store(long v) { seen = static_cast<int>(v); }
using IntTaker = void (*)(int);
int main() {
  IntTaker p = reinterpret_cast<IntTaker>(&store);
  p(4);
  return 0;
}
