// tidal-rules: SHF-3
// tidal-expect: reject
enum class E { A = 1 };
auto r = E::A << 1;
