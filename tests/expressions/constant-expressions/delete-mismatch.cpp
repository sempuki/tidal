// tidal-rules: CE-7
// tidal-expect: reject
constexpr int f() {
  int* a = new int[2]{1, 2};
  delete a;
  return 0;
}
static_assert(f() == 0);
