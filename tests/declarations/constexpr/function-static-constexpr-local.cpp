// tidal-rules: CX-3
// tidal-expect: accept
constexpr int f() {
  static constexpr int table[] = {1, 2, 3};
  return table[2];
}
static_assert(f() == 3);
