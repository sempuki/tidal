// tidal-rules: NEW-5
// tidal-expect: reject
// tidal-error-gcc: deallocation of already deallocated storage
// tidal-error-clang: delete of pointer that has already been deleted
// tidal-error-edg: deallocation of storage that was not dynamically allocated
constexpr bool f() {
  int* p = new int;
  delete p;
  delete p;
  return true;
}
static_assert(f());
