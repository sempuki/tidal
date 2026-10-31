// tidal-rules: JMP-13
// tidal-expect: reject
// tidal-error-clang: non-void function .f. should return a value
// tidal-error-gcc: return-statement with no value
// tidal-error-edg: non-void function .f. should return a value
int f() {
  return;
}
