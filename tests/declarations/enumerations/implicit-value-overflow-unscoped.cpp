// tidal-rules: ENUM-5
// tidal-expect: accept
enum E { A = 2147483647, B };
static_assert(static_cast<long long>(B) == 2147483648LL);
