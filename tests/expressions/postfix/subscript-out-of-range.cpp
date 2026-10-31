// tidal-rules: PFX-4
// tidal-expect: reject
// tidal-error-gcc: outside the bounds of array
constexpr int f() {
  int a[2] = {1, 2};
  return a[2];
}
static_assert(f() == 0);
