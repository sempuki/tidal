// tidal-rules: ENUM-1
// tidal-expect: accept
enum class E { A = 4, B = A * 2 };
static_assert(static_cast<int>(E::B) == 8);
