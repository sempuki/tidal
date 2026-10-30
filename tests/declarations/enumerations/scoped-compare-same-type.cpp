// tidal-rules: ENUM-3
// tidal-expect: accept
enum class E { A, B };
static_assert(E::A < E::B);
static_assert(E::A != E::B);
