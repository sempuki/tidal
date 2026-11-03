// tidal-rules: ASN-10
// tidal-expect: reject
// tidal-error-clang: address of bit-field requested
// tidal-error-gcc: take address of bit-field
// tidal-error-edg: address of a bit field
// tidal-note: A compound assignment to a bit-field. The result is the bit-field.
struct S { unsigned bf : 4; };
auto f(S& s) { return &(s.bf += 1); }
