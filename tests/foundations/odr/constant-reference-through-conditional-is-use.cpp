// tidal-rules: ODR-3 ODR-8
// tidal-expect: link-fail
// tidal-expect-gcc-16-o2: run
// tidal-expect-clang-22-o2: run
// tidal-error-clang: undefined reference to .Limits::top
// tidal-error-gcc: undefined reference to .Limits::top
// tidal-note: The reference binds to the result of ?:, an lvalue that can be Limits::top. Binding a reference is a use, so Limits::top needs a definition, and it has none. At -O2, GCC and Clang read the value at translation time and emit no reference to Limits::top, so the program links (ODR-8 needs no diagnostic).
struct Limits { static const int top = 5; static const int low = 1; };
int pick(bool wide) {
  const int& r = wide ? Limits::top : Limits::low;
  return r;
}
int main(int argc, char**) { return pick(argc > 0) == 5 ? 0 : 1; }
