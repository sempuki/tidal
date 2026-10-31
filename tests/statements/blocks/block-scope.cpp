// tidal-rules: STMT-2 NAM-8
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .inner
// tidal-error-gcc: inner. was not declared in this scope
// tidal-error-edg: identifier .inner. is undefined
int f() {
  { int inner = 1; }
  return inner;
}
