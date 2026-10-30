// tidal-rules: COND-6
// tidal-expect: accept
constexpr int pick(int v) { return v == 0 ? 10 : v == 1 ? 11 : 12; }
static_assert(pick(0) == 10 && pick(1) == 11 && pick(2) == 12);
static_assert((false ? 1 : true ? 2 : 3) == 2);
