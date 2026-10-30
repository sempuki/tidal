// tidal-rules: CV-5
// tidal-expect: reject
constexpr int f() {
  const int x = 1;
  const_cast<int&>(x) = 2;
  return x;
}
static_assert(f() == 2);
