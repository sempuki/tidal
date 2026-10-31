// tidal-rules: LOOP-4
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .i
// tidal-error-gcc: i. was not declared in this scope
// tidal-error-edg: identifier .i. is undefined
int f() {
  for (int i = 0; i < 2; ++i) {}
  return i;
}
