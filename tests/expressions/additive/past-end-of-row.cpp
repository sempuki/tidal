// tidal-rules: ADD-10
// tidal-expect: reject
// tidal-error-gcc: outside the bounds of array
// tidal-error-clang: cannot refer to element .* of array of
// tidal-error-edg: cannot access position .* in array of
// tidal-note: Gap review 2026-10-30.
constexpr int f() {
  int a[2][2] = {{1, 2}, {3, 4}};
  int* p = &a[0][0];
  return *(p + 3);
}
static_assert(f() == 4);
