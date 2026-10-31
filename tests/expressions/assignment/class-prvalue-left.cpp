// tidal-rules: ASN-2
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31 (third).
struct S {};
void f() { S{} = S{}; }
