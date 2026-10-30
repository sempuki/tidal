// tidal-rules: NEW-2
// tidal-expect: accept
constexpr bool check() {
  int* p = new int[]{1, 2, 3};
  bool ok = p[2] == 3;
  delete[] p;
  return ok;
}
static_assert(check());
