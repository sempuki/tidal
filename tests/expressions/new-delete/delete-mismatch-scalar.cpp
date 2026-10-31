// tidal-rules: NEW-5
// tidal-expect: reject
// tidal-error-gcc: array deallocation of object allocated with non-array allocation
constexpr bool f() {
  int* p = new int;
  delete[] p;
  return true;
}
static_assert(f());
