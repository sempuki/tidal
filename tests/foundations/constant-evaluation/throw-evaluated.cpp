// tidal-rules: CE-8
// tidal-expect: reject
// tidal-error-gcc: <throw-expression>. is not a constant expression
constexpr int f(int v) {
  if (v < 0) throw v;
  return v;
}
static_assert(f(-1) == 0);
