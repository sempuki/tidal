// tidal-rules: MUL-11
// tidal-expect: reject
constexpr double q(double a, double b) { return a / b; }
static_assert(q(1.0, 0.0) != 0);
