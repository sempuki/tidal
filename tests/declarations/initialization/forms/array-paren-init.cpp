// tidal-rules: INIT-5
// tidal-expect: accept
constexpr int a[3](1, 2);
static_assert(a[0] == 1 && a[1] == 2 && a[2] == 0);
constexpr int b[2](1.5, 2);
static_assert(b[0] == 1);
