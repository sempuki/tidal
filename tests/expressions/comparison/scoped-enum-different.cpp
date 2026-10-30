// tidal-rules: CMP-10
// tidal-expect: reject
enum class E { A };
enum class F { A };
bool b = E::A == F::A;
