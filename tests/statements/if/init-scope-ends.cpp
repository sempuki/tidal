// tidal-rules: IF-9 NAM-8
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .a
// tidal-error-gcc: a. was not declared in this scope
// tidal-error-edg: identifier .a. is undefined
int f() {
  if (int a = 1; a) {}
  return a;
}
