// tidal-rules: MUL-10
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Q-0003. EDG 7.0 accepts INT_MIN / -1 during constant evaluation.
constexpr int int_max = int(~0u >> 1);
constexpr int q(int a, int b) { return a / b; }
static_assert(q(-int_max - 1, -1) != 0);
