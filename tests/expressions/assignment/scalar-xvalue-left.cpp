// tidal-rules: ASN-2
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
void f(int i) { static_cast<int&&>(i) = 1; }
