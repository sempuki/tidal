// tidal-rules: BIT-3
// tidal-expect: reject
enum class Flags { A = 1, B = 2 };
auto r = Flags::A | Flags::B;
