// tidal-rules: UN-4
// tidal-expect: reject
// tidal-error-gcc: overflow in constant expression
// tidal-error-clang: outside the range of representable values of type .int
// tidal-error-edg: value exceeds range of .int
constexpr int neg(int v) { return -v; }
static_assert(neg(-2147483647 - 1) != 0);
