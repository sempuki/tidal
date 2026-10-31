// tidal-rules: PFX-8
// tidal-expect: reject
// tidal-error-gcc: expression .0. does not designate a .constexpr. function
constexpr int f() {
  int (*p)() = nullptr;
  return p();
}
static_assert(f() == 0);
