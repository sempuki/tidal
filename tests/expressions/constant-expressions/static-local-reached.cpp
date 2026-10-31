// tidal-rules: CE-17
// tidal-expect: reject
// tidal-error-gcc: with static storage duration
// tidal-error-clang: control flows through the definition of a static variable
// tidal-error-edg: declaration of a variable with static or thread storage duration
// tidal-note: Gap review 2026-10-31.
constexpr int f(bool b) { if (b) { static int n = 0; return ++n; } return 1; }
static_assert(f(true) == 1);
