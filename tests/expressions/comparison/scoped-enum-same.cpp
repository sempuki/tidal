// tidal-rules: CMP-10
// tidal-expect: accept
enum class E { A, B };
static_assert(E::A < E::B && E::A != E::B);
