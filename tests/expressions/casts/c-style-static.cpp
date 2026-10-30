// tidal-rules: CAST-9
// tidal-expect: accept
static_assert((int)3.9 == 3);
static_assert(int(3.9) == 3);
enum class E { A = 1 };
static_assert((int)E::A == 1);
