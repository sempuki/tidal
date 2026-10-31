// tidal-rules: MUL-10
// tidal-expect: reject
// tidal-error-gcc: overflow in constant expression
constexpr int int_max = int(~0u >> 1);
constexpr int m(int a, int b) { return a * b; }
static_assert(m(int_max, 2) != 0);
