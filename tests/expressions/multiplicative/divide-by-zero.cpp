// tidal-rules: MUL-9
// tidal-expect: reject
// tidal-error-gcc: \(1 / 0\). is not a constant expression
constexpr int q(int a, int b) { return a / b; }
static_assert(q(1, 0) == 0);
