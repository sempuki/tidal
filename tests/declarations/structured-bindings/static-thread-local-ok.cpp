// tidal-rules: SB-11
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct P { int a, b; };
void f() { static auto [x, y] = P{1, 2}; thread_local auto [u, v] = P{3, 4}; (void)x; (void)u; }
