// tidal-rules: TR-8
// tidal-expect: reject
constexpr int f() { int ignored = 1 / 0; return 5; }
static_assert(f() == 5);
