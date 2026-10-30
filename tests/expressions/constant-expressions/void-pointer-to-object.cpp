// tidal-rules: CE-4
// tidal-expect: reject
constexpr int f() {
  int x = 7;
  void* v = &x;
  return *static_cast<int*>(v);
}
static_assert(f() == 7);
