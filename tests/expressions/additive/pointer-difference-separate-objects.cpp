// tidal-rules: ADD-11
// tidal-expect: reject
// tidal-error-clang: unrelated objects|subexpression not valid in a constant expression
// tidal-error-gcc: is not a constant expression
// tidal-error-edg: expression must have a constant value
constexpr int f() {
  int first = 1;
  int second = 2;
  return &second - &first;
}
constexpr int d = f();
