// tidal-rules: COND-3
// tidal-expect: reject
// tidal-error-clang: expression is not assignable
// tidal-error-gcc: lvalue required as left operand of assignment
// tidal-error-edg: must be a modifiable lvalue
int f(bool c) {
  int x = 1;
  (c ? x : 2) = 3;
  return x;
}
