// tidal-rules: MUL-9
// tidal-expect: reject
// tidal-error-gcc: \(1 % 0\). is not a constant expression
// tidal-error-clang: division by zero
// tidal-error-edg: division by zero
constexpr int r(int a, int b) { return a % b; }
static_assert(r(1, 0) == 0);
