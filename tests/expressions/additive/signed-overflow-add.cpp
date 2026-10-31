// tidal-rules: ADD-9
// tidal-expect: reject
// tidal-error-gcc: overflow in constant expression
constexpr int int_max = int(~0u >> 1);
constexpr int add(int a, int b) { return a + b; }
static_assert(add(int_max, 1) != 0);
