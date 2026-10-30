// tidal-rules: ENUM-3
// tidal-expect: accept
enum class E { A = 7 };
static_assert(static_cast<int>(E::A) == 7);
static_assert(static_cast<E>(7) == E::A);
