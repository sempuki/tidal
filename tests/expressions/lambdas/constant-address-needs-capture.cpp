// tidal-rules: LAM-17
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
void f() { const int c = 4; auto l = [] { const int* p = &c; return *p; }; }
