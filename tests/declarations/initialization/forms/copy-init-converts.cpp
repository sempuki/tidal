// tidal-rules: INIT-4
// tidal-expect: accept
constexpr int a = 3.7;
constexpr int b(3.7);
constexpr unsigned char c = 300;
constexpr bool d = 2;
static_assert(a == 3 && b == 3);
static_assert(c == 44);
static_assert(d == true);
