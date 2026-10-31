// tidal-rules: ODR-2
// tidal-expect: run
// tidal-expect-clang: reject
// tidal-error-clang: cannot be implicitly captured
// tidal-note: Q-0129. GCC and EDG treat the copy as a read of the constant. Clang treats the copy as a use of the variable.
int main() {
  struct Size { int w; };
  constexpr Size size{4};
  auto copy = [] { return size; };
  return copy().w - 4;
}
