// tidal-rules: JMP-14
// tidal-expect: reject
// tidal-error-clang: undeclared label .nowhere
// tidal-error-gcc: label .nowhere. used but not defined
// tidal-error-edg: label .nowhere. was referenced but not defined
void f() {
  goto nowhere;
}
