// tidal-rules: TR-5
// tidal-expect: reject
// tidal-error-gcc: \(1 / 0\). is not a constant expression
constexpr int f(int d) { return 1 / d; }
static_assert(f(0) == 0);
