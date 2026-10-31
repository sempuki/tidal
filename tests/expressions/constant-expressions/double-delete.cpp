// tidal-rules: CE-7
// tidal-expect: reject
// tidal-error-gcc: deallocation of already deallocated storage
constexpr int f() {
  int* p = new int(1);
  delete p;
  delete p;
  return 0;
}
static_assert(f() == 0);
