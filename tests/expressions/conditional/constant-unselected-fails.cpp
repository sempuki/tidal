// tidal-rules: COND-7
// tidal-expect: accept
int runtime();
constexpr int f(bool b) { return b ? 1 : runtime(); }
static_assert(f(true) == 1);
constexpr int g = true ? 4 : 1 / 0;
