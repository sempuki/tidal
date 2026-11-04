// tidal-rules: VC-2
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31 (third).
struct P { int a, b; };
P make();
void f() { make() = {1, 2}; }
