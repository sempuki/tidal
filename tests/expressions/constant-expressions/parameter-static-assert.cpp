// tidal-rules: CE-30 CE-3
// tidal-expect: reject
// tidal-error-gcc: .n. is not a constant expression
// tidal-error-clang: static assertion expression is not an integral constant expression
// tidal-error-edg: parameter .n.
// tidal-note: The static_assert is checked once, at its place in the function, outside any call. A call from a constant expression does not make n usable there.
constexpr int checked(int n) {
  static_assert(n > 0);
  return n;
}
static_assert(checked(1) == 1);
