// tidal-rules: NEW-32
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: constructor call on null pointer
// tidal-note: Run-time check with the sanitizer (decision 0028).
using size = decltype(sizeof 0);
void* operator new(size, int) { return nullptr; }
struct Gadget { int value; Gadget(int v) : value(v) {} };
int main(int argc, char**) {
  Gadget* g = new (argc) Gadget(argc);
  return g == nullptr ? 0 : 1;
}
