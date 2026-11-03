// tidal-rules: CE-8
// tidal-expect: reject
// tidal-error-clang: subexpression not valid in a constant expression
// tidal-error-gcc: <throw-expression>. is not a constant expression
// tidal-error-edg: expression must have a constant value
constexpr int check(int v) noexcept {
  if (v < 0) throw v;
  return v;
}
static_assert(check(-1) == 0);
