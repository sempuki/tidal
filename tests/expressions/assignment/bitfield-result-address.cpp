// tidal-rules: ASN-10
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
struct S { int bf : 3; };
auto f(S& s) { return &(s.bf = 1); }
