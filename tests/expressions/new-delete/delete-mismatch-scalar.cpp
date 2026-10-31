// tidal-rules: NEW-5
// tidal-expect: reject
// tidal-error-gcc: array deallocation of object allocated with non-array allocation
// tidal-error-clang: array delete used to delete pointer to non-array object
// tidal-error-edg: deallocation does not match allocation kind
constexpr bool f() {
  int* p = new int;
  delete[] p;
  return true;
}
static_assert(f());
