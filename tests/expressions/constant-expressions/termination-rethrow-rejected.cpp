// tidal-rules: CE-8
// tidal-expect: reject
// tidal-error-clang: subexpression not valid in a constant expression
// tidal-error-gcc: <throw-expression>. is not a constant expression
// tidal-error-edg: expression must have a constant value
// tidal-note: The evaluation reaches the rethrow, and no exception is being handled. A run would terminate (UNW-5).
constexpr int pass_on(bool rethrow) {
  if (rethrow) throw;
  return 1;
}
static_assert(pass_on(true) == 1);
