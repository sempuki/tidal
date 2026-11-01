// tidal-rules: COMMA-3
// tidal-expect: accept
template <int N> constexpr int value = N;
static_assert(value<(1, 2)> == 2);
constexpr int step() { int i = 0; int j = (++i, i * 10); return j; }
static_assert(step() == 10);
