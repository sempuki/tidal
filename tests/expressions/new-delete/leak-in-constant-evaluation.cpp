// tidal-rules: NEW-6
// tidal-expect: reject
// tidal-error-gcc: allocated storage has not been deallocated
// tidal-error-clang: allocation performed here was not deallocated
// tidal-error-edg: dynamic allocations .* were not deallocated
constexpr bool f() {
  int* p = new int;
  return p != nullptr;
}
static_assert(f());
