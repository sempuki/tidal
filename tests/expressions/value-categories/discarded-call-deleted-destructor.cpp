// tidal-rules: VC-11
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
struct D { ~D() = delete; };
D make();
void f() { make(); }
