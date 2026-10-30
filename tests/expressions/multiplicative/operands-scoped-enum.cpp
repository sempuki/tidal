// tidal-rules: MUL-3
// tidal-expect: reject
enum class Scoped { One = 1 };
auto r = Scoped::One * 2;
