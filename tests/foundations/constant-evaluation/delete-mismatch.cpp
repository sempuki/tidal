// tidal-rules: CE-7
// tidal-expect: reject
// tidal-error-gcc: non-array deallocation of object allocated with array allocation
// tidal-error-clang: non-array delete used to delete pointer to array object
// tidal-error-edg: deallocation does not match allocation kind
constexpr int f() {
  int* a = new int[2]{1, 2};
  delete a;
  return 0;
}
static_assert(f() == 0);
