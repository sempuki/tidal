// tidal-rules: JMP-7
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
void f() { while (true) { auto l = [] { break; }; l(); } }
