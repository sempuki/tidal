// tidal-rules: TR-8
// tidal-expect: reject
constexpr int f(int d) { int r = 5; if (d == 0) { int* p = nullptr; r = *p; } return r; }
static_assert(f(1) == 5);
static_assert(f(0) == 5);
