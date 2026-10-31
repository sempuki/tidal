// tidal-rules: ADD-11
// tidal-expect: reject
// tidal-error-gcc: \(& b\[0\]\) - \(& a\[0\]\)
constexpr int a[2] = {};
constexpr int b[2] = {};
constexpr long d = &b[0] - &a[0];
