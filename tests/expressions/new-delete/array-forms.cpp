// tidal-rules: NEW-2
// tidal-expect: accept
constexpr bool check() {
  int* a = new int[3]{1, 2};
  int* b = new int[2]();
  bool ok = a[0] == 1 && a[1] == 2 && a[2] == 0 && b[0] == 0 && b[1] == 0;
  delete[] a; delete[] b;
  return ok;
}
static_assert(check());
