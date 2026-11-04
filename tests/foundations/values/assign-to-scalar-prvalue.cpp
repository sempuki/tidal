// tidal-rules: VC-12
// tidal-expect: reject
// tidal-error-clang: expression is not assignable
// tidal-error-gcc: lvalue required as left operand of assignment
// tidal-error-edg: must be a modifiable lvalue
// tidal-note: Gap review 2026-10-31 (third).
int make();
void f() { make() = 1; }
