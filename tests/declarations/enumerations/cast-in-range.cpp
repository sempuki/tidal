// tidal-rules: ENUM-6
// tidal-expect: accept
enum E { A = 1, B = 4 };
constexpr E e = static_cast<E>(7);
static_assert(static_cast<int>(e) == 7);
