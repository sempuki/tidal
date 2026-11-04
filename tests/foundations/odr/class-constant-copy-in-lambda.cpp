// tidal-rules: ODR-2 ODR-3
// tidal-expect: reject
// tidal-expect-gcc: run
// tidal-expect-edg: accept
// tidal-error-clang: cannot be implicitly captured
// tidal-note: Q-0129. The copy binds the parameter of the copy constructor to size, so the copy uses size, and the lambda must capture it. Clang rejects. GCC and EDG treat the copy as a read of the constant.
int main() {
  struct Size { int w; };
  constexpr Size size{4};
  auto copy = [] { return size; };
  return copy().w - 4;
}
