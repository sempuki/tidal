// tidal-rules: ENUM-12
// tidal-expect: accept
// tidal-note: In a scoped enumeration, the enumerator's own name in its value expression refers to the variable outside.
constexpr int value = 12;
enum class E { value = value + 1, next = value * 2 };
static_assert(static_cast<int>(E::value) == 13);
static_assert(static_cast<int>(E::next) == 26);
