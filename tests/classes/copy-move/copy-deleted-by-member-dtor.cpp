// tidal-rules: CPY-7
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct M { ~M() = delete; };
struct S { M m; S(); S(const S&) = default; };
void f(S& a) { S b(a); (void)b; }
