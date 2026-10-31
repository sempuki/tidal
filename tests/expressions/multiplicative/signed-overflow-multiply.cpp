// tidal-rules: MUL-10
// tidal-expect: reject
// tidal-error-gcc: overflow in constant expression
// tidal-error-clang: outside the range of representable values of type .int
// tidal-error-edg: value exceeds range of .int
constexpr int int_max = int(~0u >> 1);
constexpr int m(int a, int b) { return a * b; }
static_assert(m(int_max, 2) != 0);
