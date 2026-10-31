// tidal-rules: JMP-10
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
struct M { M() {} M(M&&) {} };
M f() { static M m; return m; }
