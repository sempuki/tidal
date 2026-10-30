// tidal-rules: ATTR-10
// tidal-expect: accept
constexpr int f(int v) {
  [[assume(v > 0)]];
  return v;
}
static_assert(f(1) == 1);
