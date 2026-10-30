// tidal-rules: CONV-4
// tidal-expect: accept
constexpr int a = 3.99;
constexpr int b = -3.99;
constexpr unsigned c = 2.5;
static_assert(a == 3 && b == -3 && c == 2);
