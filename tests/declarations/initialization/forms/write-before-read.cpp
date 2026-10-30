// tidal-rules: INIT-1
// tidal-expect: accept
constexpr int f() {
  int x;
  x = 3;
  return x;
}
static_assert(f() == 3);
