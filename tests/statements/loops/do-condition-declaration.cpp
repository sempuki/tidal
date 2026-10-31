// tidal-rules: LOOP-6
// tidal-expect: reject
// tidal-error-gcc: expected primary-expression before .int
// tidal-error-edg: type name is not allowed
void f() {
  do {} while (int x = 0);
}
