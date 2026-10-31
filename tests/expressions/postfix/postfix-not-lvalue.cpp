// tidal-rules: PFX-13
// tidal-expect: reject
// tidal-error-clang: expression is not assignable
// tidal-error-gcc: lvalue required as increment operand
// tidal-error-edg: must be a modifiable lvalue
void f(int i) { (i++)++; }
