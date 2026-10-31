// tidal-rules: CE-17
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct NL { NL() {} ~NL() {} };
constexpr int f(bool b) { if (b) { NL n; return 0; } return 1; }
static_assert(f(false) == 1);
