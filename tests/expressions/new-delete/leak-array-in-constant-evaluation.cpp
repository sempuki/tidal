// tidal-rules: NEW-6
// tidal-expect: reject
// tidal-error-clang: allocation performed here was not deallocated
// tidal-error-gcc: allocated storage has not been deallocated
// tidal-error-edg: dynamic allocations .* were not deallocated
// tidal-note: The evaluation reads the array and ends without delete[].
constexpr int first() {
  int* values = new int[3]{4, 5, 6};
  return values[0];
}
static_assert(first() == 4);
