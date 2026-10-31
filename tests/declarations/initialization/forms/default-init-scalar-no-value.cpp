// tidal-rules: INIT-9 INIT-1
// tidal-expect: reject
// tidal-error-clang: uninitialized
// tidal-error-gcc: uninitialized|is not usable in a constant expression
// tidal-error-edg: uninitialized
constexpr int read() {
  int v;
  return v;
}
static_assert(read() == 0);
