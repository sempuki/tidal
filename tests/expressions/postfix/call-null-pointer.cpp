// tidal-rules: PFX-8
// tidal-expect: reject
// tidal-error-gcc: expression .0. does not designate a .constexpr. function
// tidal-error-clang: evaluates to a null function pointer
// tidal-error-edg: callee is null
constexpr int f() {
  int (*p)() = nullptr;
  return p();
}
static_assert(f() == 0);
