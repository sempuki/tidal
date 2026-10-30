// tidal-rules: CAST-1
// tidal-expect: accept
enum class E { A = 2 };
static_assert(static_cast<int>(E::A) == 2);
static_assert(static_cast<E>(2) == E::A);
