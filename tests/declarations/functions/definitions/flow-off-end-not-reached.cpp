// tidal-rules: FN-3
// tidal-expect: accept
constexpr int f(bool b) {
  if (b) return 1;
}
static_assert(f(true) == 1);
