// tidal-rules: JMP-5
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
void f() { out: ; auto l = [] { goto out; }; l(); }
