// tidal-rules: CE-1
// tidal-expect: accept
constexpr int f() {
  int* p = new int(5);
  int v = *p;
  delete p;
  int* a = new int[3]{1, 2, 3};
  v += a[2];
  delete[] a;
  return v;
}
static_assert(f() == 8);
