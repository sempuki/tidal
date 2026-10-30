// tidal-rules: ENUM-4
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
enum Flag : bool { Off, On };
static_assert(static_cast<bool>(On));
enum Cv : const int { X };
