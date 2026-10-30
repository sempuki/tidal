// tidal-rules: ADD-7
// tidal-expect: accept
constexpr int a[5] = {};
static_assert(&a[4] - &a[1] == 3);
static_assert(&a[1] - &a[4] == -3);
