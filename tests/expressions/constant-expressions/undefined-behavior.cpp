// tidal-rules: CE-9
// tidal-expect: reject
constexpr int f(int a) { return a / 0; }
static_assert(f(1) == 0);
