// tidal-rules: CE-7
// tidal-expect: reject
// tidal-error-gcc: allocated storage has not been deallocated
// tidal-error-clang: allocation performed here was not deallocated
// tidal-error-edg: dynamic allocations .* were not deallocated
constexpr int f() {
  int* p = new int(5);
  return *p;
}
static_assert(f() == 5);
