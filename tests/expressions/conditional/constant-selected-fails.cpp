// tidal-rules: COND-7
// tidal-expect: reject
// tidal-error-gcc: call to non-.constexpr. function
int runtime();
constexpr int f(bool b) { return b ? 1 : runtime(); }
static_assert(f(false) == 1);
