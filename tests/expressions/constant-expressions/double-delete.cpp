// tidal-rules: CE-7
// tidal-expect: reject
// tidal-error-gcc: deallocation of already deallocated storage
// tidal-error-clang: delete of pointer that has already been deleted
// tidal-error-edg: deallocation of storage that was not dynamically allocated
constexpr int f() {
  int* p = new int(1);
  delete p;
  delete p;
  return 0;
}
static_assert(f() == 0);
