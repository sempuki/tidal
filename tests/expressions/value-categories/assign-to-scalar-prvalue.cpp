// tidal-rules: VC-2
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
int make();
void f() { make() = 1; }
