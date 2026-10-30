// tidal-rules: PFX-8
// tidal-expect: reject
constexpr int f() {
  int (*p)() = nullptr;
  return p();
}
static_assert(f() == 0);
