// tidal-rules: FN-3
// tidal-expect: reject
constexpr int f(bool b) {
  if (b) return 1;
}
static_assert(f(false) == 0);
