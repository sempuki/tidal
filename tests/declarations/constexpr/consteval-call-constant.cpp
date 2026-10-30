// tidal-rules: CEV-1
// tidal-expect: accept
consteval int square(int v) { return v * v; }
constexpr int a = square(3);
int b = square(4);
static_assert(a == 9);
