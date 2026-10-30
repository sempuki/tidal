// tidal-rules: ENUM-4
// tidal-expect: accept
enum class E { A };
static_assert(sizeof(E) == sizeof(int));
