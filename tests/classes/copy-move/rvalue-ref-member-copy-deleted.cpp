// tidal-rules: CPY-7
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
int g;
struct S { int&& r; };
S make();
void f(S& a) { S b(a); }
