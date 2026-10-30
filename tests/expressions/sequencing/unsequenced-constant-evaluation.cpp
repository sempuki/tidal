// tidal-rules: SEQ-4
// tidal-expect: accept
constexpr int f() {
  int i = 1;
  int j = i + 1;
  return j;
}
static_assert(f() == 2);
