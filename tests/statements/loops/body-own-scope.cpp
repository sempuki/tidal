// tidal-rules: LOOP-4 NAM-8
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .x
// tidal-error-gcc: x. was not declared in this scope
// tidal-error-edg: identifier .x. is undefined
int f(int n) {
  while (n-- > 0) int x = n;
  return x;
}
