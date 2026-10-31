// tidal-rules: RET-1
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
auto f() { if constexpr (false) { return 1.5; } else { return 2; } }
int (*p)() = f;
