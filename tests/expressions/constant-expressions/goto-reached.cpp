// tidal-rules: CE-20
// tidal-expect: reject
// tidal-error-gcc: .goto. is not a constant expression
// tidal-error-edg: goto. cannot be executed in constexpr contexts
// tidal-note: Gap review 2026-10-31.
constexpr int f(bool b) { if (b) { goto done; } return 1; done: return 2; }
static_assert(f(true) == 2);
