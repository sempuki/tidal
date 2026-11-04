// tidal-rules: VC-12
// tidal-expect: reject
// tidal-error-clang: expression is not assignable
// tidal-error-gcc: lvalue required as left operand of assignment
// tidal-error-edg: must be a modifiable lvalue
int val();
void f() { val() = 1; }
