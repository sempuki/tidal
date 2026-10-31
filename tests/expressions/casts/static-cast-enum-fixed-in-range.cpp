// tidal-rules: CAST-1
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
enum Fixed : int { a };
constexpr Fixed f = static_cast<Fixed>(9);
