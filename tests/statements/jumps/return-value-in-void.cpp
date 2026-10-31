// tidal-rules: JMP-13
// tidal-expect: reject
// tidal-error-clang: void function .f. should not return a value
// tidal-error-gcc: return-statement with a value, in function returning .void
// tidal-error-edg: return value type does not match the function type
void f() {
  return 1;
}
