// tidal-rules: CE-7
// tidal-expect: reject
constexpr int f() {
  int* p = new int(5);
  return *p;
}
static_assert(f() == 5);
