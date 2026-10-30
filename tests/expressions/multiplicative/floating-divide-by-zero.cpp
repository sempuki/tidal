// tidal-rules: MUL-11
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Q-0002. EDG 7.0 accepts floating division by zero during constant evaluation.
constexpr double q(double a, double b) { return a / b; }
static_assert(q(1.0, 0.0) != 0);
