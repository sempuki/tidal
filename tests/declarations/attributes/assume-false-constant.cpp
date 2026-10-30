// tidal-rules: ATTR-10
// tidal-expect: reject
// tidal-expect-clang-18: accept
// tidal-note: Q-0020. Clang 18 doesn't implement [[assume]] and ignores it as an unknown attribute.
constexpr int f(int v) {
  [[assume(v > 0)]];
  return v;
}
static_assert(f(-1) == -1);
