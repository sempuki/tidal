// tidal-rules: INIT-5
// tidal-expect: accept
constexpr int a[4] = {1, 2};
static_assert(a[0] == 1 && a[1] == 2 && a[2] == 0 && a[3] == 0);
constexpr int b[3] = {};
static_assert(b[0] == 0 && b[2] == 0);
