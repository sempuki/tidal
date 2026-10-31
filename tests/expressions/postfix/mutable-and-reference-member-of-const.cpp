// tidal-rules: PFX-10
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
int g;
struct S { mutable int m = 0; int& r = g; };
void f(const S& s) { s.m = 1; s.r = 2; }
