// tidal-rules: MUL-19
// tidal-expect: reject
// tidal-error-clang: expression is not assignable
// tidal-error-gcc: lvalue required as left operand of assignment
// tidal-error-edg: must be a modifiable lvalue
void f(int a, int b) {
  a * b = 4;
}
