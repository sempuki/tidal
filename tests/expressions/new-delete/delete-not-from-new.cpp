// tidal-rules: NEW-5
// tidal-expect: reject
constexpr bool f() {
  int x = 0;
  int* p = &x;
  delete p;
  return true;
}
static_assert(f());
