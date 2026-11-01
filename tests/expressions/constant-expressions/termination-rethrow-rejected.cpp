// tidal-rules: CE-31 CE-8
// tidal-expect: reject
// tidal-error-clang: subexpression not valid in a constant expression
// tidal-error-gcc: <throw-expression>. is not a constant expression
// tidal-error-edg: expression must have a constant value
constexpr int pass_on(bool handling) {
  if (!handling) throw;
  return 1;
}
static_assert(pass_on(false) == 1);
