// tidal-rules: CE-3 CE-14
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: temporary.* is not usable in a constant expression
// tidal-note: Q-0193
struct Pair { int first; int second; };
constexpr const Pair& pair = Pair{1, 2};
static_assert(pair.second == 2);
