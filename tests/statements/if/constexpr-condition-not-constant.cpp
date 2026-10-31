// tidal-rules: IF-16
// tidal-expect: reject
// tidal-error-clang: constexpr if condition is not a constant expression
// tidal-error-gcc: b. is not a constant expression
// tidal-error-edg: expression must have a constant value
void f(bool b) {
  if constexpr (b) {}
}
