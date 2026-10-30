// tidal-rules: ENUM-5
// tidal-expect: accept
enum Small { S = 1 };
enum Big { L = 1LL << 40 };
enum Neg { N = -1, P = 1 };
static_assert(sizeof(Big) >= 8);
static_assert(static_cast<long long>(L) == 1LL << 40);
static_assert(N < 0);
