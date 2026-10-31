// tidal-rules: NEW-3
// tidal-expect: reject
// tidal-error-gcc: content of uninitialized storage is not usable
constexpr int f() {
  int* p = new int;
  int v = *p;
  delete p;
  return v;
}
static_assert(f() == 0);
