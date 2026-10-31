// tidal-rules: NEW-5
// tidal-expect: reject
// tidal-error-gcc: deallocation of storage that was not previously allocated
constexpr bool f() {
  int x = 0;
  int* p = &x;
  delete p;
  return true;
}
static_assert(f());
