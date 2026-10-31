// tidal-rules: TR-10
// tidal-expect: reject
// tidal-error-gcc: \(1 / 0\). is not a constant expression
// tidal-error-clang: division by zero
// tidal-error-edg: division by zero
constexpr int f(int d) { return 1 / d; }
static_assert(f(0) == 0);
