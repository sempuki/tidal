// tidal-rules: NEW-5
// tidal-expect: reject
// tidal-error-gcc: non-array deallocation of object allocated with array allocation
// tidal-error-clang: non-array delete used to delete pointer to array object
// tidal-error-edg: deallocation does not match allocation kind
constexpr bool f() {
  int* p = new int[2];
  delete p;
  return true;
}
static_assert(f());
