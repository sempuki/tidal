// tidal-rules: ASN-2
// tidal-expect: reject
// tidal-error-clang: expression is not assignable
// tidal-error-gcc: using rvalue as lvalue
// tidal-error-edg: must be a modifiable lvalue
// tidal-note: Gap review 2026-10-31 (third).
void f(int i) { static_cast<int&&>(i) = 1; }
