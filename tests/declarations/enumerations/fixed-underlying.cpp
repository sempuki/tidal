// tidal-rules: ENUM-4
// tidal-expect: accept
enum E : unsigned char { A = 255 };
enum class F : long long { B = 1LL << 40 };
static_assert(sizeof(E) == 1);
static_assert(sizeof(F) == sizeof(long long));
