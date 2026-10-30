// tidal-rules: COMMA-3
// tidal-expect: accept
constexpr int f() {
  int a = 0;
  int r = (a = 1, a += 2, a * 10);
  return r;
}
static_assert(f() == 30);
