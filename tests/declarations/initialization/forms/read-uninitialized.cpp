// tidal-rules: INIT-1
// tidal-expect: reject
constexpr int f() {
  int x;
  return x;
}
static_assert(f() == 0);
