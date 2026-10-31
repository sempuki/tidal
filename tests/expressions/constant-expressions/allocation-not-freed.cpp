// tidal-rules: CE-7
// tidal-expect: reject
// tidal-error-gcc: allocated storage has not been deallocated
constexpr int f() {
  int* p = new int(5);
  return *p;
}
static_assert(f() == 5);
