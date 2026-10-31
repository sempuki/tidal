// tidal-rules: CONV-8
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31 (third).
struct S { int m; void f(); };
int S::* pm = 0;
void (S::* pf)() = 0;
