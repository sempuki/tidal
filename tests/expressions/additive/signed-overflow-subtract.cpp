// tidal-rules: ADD-9
// tidal-expect: reject
// tidal-error-gcc: overflow in constant expression
// tidal-error-clang: outside the range of representable values of type .int
// tidal-error-edg: value exceeds range of .int
constexpr int int_max = int(~0u >> 1);
constexpr int sub(int a, int b) { return a - b; }
static_assert(sub(-int_max - 1, 1) != 0);
