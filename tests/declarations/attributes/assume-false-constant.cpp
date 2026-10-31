// tidal-rules: ATTR-10
// tidal-expect: reject
// tidal-expect-clang-18: accept
// tidal-error-clang: not an integral constant expression
// tidal-error-gcc: failed .assume. attribute assumption
// tidal-error-edg: must have a constant value
// tidal-note: Q-0020. Clang 18 doesn't implement [[assume]] and ignores it as an unknown attribute.
constexpr int f(int v) {
  [[assume(v > 0)]];
  return v;
}
static_assert(f(-1) == -1);
