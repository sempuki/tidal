// tidal-rules: INIT-1
// tidal-expect: reject
// tidal-error-gcc: value of .x. is not usable in a constant expression
constexpr int f() {
  int x;
  return x;
}
static_assert(f() == 0);
