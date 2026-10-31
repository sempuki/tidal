// tidal-rules: NEW-6
// tidal-expect: reject
// tidal-error-gcc: allocated storage has not been deallocated
constexpr bool f() {
  int* p = new int;
  return p != nullptr;
}
static_assert(f());
