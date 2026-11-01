// tidal-rules: NEW-2
// tidal-expect: accept
constexpr bool check() {
  int* p = new int[3](1, 2);
  bool ok = p[0] == 1 && p[1] == 2 && p[2] == 0;
  delete[] p;
  return ok;
}
static_assert(check());
