// tidal-rules: PFX-19
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-note: The implicit conversion of CONV-10 drops noexcept, and the call through the result is valid.
int calls = 0;
int count() noexcept { return ++calls; }
int main() {
  int (*plain)() = count;
  int (&ref)() = count;
  return plain() == 1 && ref() == 2 ? 0 : 1;
}
