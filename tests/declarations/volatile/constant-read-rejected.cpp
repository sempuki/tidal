// tidal-rules: VOL-1
// tidal-expect: reject
constexpr int f() { volatile int v = 1; return v; }
static_assert(f() == 1);
