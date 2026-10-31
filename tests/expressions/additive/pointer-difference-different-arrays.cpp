// tidal-rules: ADD-11
// tidal-expect: reject
// tidal-error-gcc: \(& b\[0\]\) - \(& a\[0\]\)
// tidal-error-edg: invalid arithmetic on non-array pointer
constexpr int a[2] = {};
constexpr int b[2] = {};
constexpr long d = &b[0] - &a[0];
