// tidal-rules: LAM-9
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
void f() { const int c = 4; auto l = [] { return c + 1; }; l(); }
