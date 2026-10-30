// tidal-rules: ADD-10
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
constexpr int f() {
  int a[2][2] = {{1, 2}, {3, 4}};
  int* p = &a[0][0];
  return *(p + 3);
}
static_assert(f() == 4);
