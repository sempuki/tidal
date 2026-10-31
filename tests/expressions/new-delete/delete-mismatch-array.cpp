// tidal-rules: NEW-5
// tidal-expect: reject
// tidal-error-gcc: non-array deallocation of object allocated with array allocation
constexpr bool f() {
  int* p = new int[2];
  delete p;
  return true;
}
static_assert(f());
