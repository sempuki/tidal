// tidal-rules: ENUM-6
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
enum Empty {};
constexpr Empty e = static_cast<Empty>(1);
static_assert(static_cast<int>(e) == 1);
