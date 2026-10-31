// tidal-rules: PFX-18
// tidal-expect: reject
// tidal-error-gcc: overflow in constant expression
// tidal-error-clang: outside the range of representable values of type .int
// tidal-error-edg: value exceeds range of .int
constexpr int f() { int i = -2147483647 - 1; i--; return i; }
static_assert(f() > 0);
