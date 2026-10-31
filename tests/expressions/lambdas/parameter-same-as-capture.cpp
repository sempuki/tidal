// tidal-rules: LAM-17
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
void f() { int x = 1; auto l = [x](int x) { return x; }; }
