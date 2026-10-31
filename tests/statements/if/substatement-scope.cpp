// tidal-rules: IF-8
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .y
// tidal-error-gcc: y. was not declared in this scope
// tidal-error-edg: identifier .y. is undefined
int f() {
  if (true) int y = 1;
  return y;
}
