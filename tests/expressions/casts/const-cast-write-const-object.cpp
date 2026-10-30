// tidal-rules: CAST-5
// tidal-expect: reject
constexpr int f() {
  const int c = 1;
  *const_cast<int*>(&c) = 2;
  return c;
}
static_assert(f() == 2);
