// tidal-rules: CE-4
// tidal-expect: reject
constexpr int f() {
  int x = 1;
  return *reinterpret_cast<char*>(&x);
}
static_assert(f() == 1);
