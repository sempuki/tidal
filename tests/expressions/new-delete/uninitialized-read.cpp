// tidal-rules: NEW-3
// tidal-expect: reject
constexpr int f() {
  int* p = new int;
  int v = *p;
  delete p;
  return v;
}
static_assert(f() == 0);
