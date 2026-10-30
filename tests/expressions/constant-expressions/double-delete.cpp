// tidal-rules: CE-7
// tidal-expect: reject
constexpr int f() {
  int* p = new int(1);
  delete p;
  delete p;
  return 0;
}
static_assert(f() == 0);
