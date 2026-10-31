// tidal-rules: LAM-11
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
void f() { int x = 1; auto a = [x] { return x; }; auto b = a; b = a; }
