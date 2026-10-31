// tidal-rules: NEW-3
// tidal-expect: reject
// tidal-error-gcc: content of uninitialized storage is not usable
// tidal-error-clang: read of uninitialized object
// tidal-error-edg: access to uninitialized object
constexpr int f() {
  int* p = new int;
  int v = *p;
  delete p;
  return v;
}
static_assert(f() == 0);
