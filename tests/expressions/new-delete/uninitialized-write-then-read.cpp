// tidal-rules: NEW-3
// tidal-expect: accept
constexpr int f() {
  int* p = new int;
  *p = 4;
  int v = *p;
  delete p;
  return v;
}
static_assert(f() == 4);
