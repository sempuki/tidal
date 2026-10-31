// tidal-rules: JMP-3
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
void nothing();
struct S { ~S() { return nothing(); } };
