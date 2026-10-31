// tidal-rules: NEW-5
// tidal-expect: reject
// tidal-error-gcc: deallocation of already deallocated storage
constexpr bool f() {
  int* p = new int;
  delete p;
  delete p;
  return true;
}
static_assert(f());
