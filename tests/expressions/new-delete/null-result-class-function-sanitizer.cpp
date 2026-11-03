// tidal-rules: NEW-32
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-error: constructor call on null pointer
// tidal-note: Run-time check with the sanitizer (decision 0028). The allocation function is a member of the class and is not non-throwing.
using size = decltype(sizeof 0);
struct Widget {
  int value;
  Widget(int v) : value(v) {}
  static void* operator new(size, int) { return nullptr; }
};
int main(int argc, char**) {
  Widget* w = new (argc) Widget(argc);
  return w == nullptr ? 0 : 1;
}
