// tidal-rules: BF-8
// tidal-expect: reject
// tidal-flags: -Werror
// tidal-note: Gap review 2026-10-31 (third).
struct S { unsigned x : 3; };
void f(S& s) { s.x = 10; }
