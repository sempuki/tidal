// tidal-rules: ASN-10
// tidal-expect: reject
// tidal-error-clang: address of bit-field requested
// tidal-error-gcc: take address of bit-field
// tidal-error-edg: address of a bit field
// tidal-note: Gap review 2026-10-31 (third).
struct S { int bf : 3; };
auto f(S& s) { return &(s.bf = 1); }
