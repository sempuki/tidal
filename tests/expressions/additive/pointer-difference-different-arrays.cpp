// tidal-rules: ADD-11
// tidal-expect: reject
constexpr int a[2] = {};
constexpr int b[2] = {};
constexpr long d = &b[0] - &a[0];
