// tidal-rules: JMP-14
// tidal-expect: reject
// tidal-error-clang: undeclared label .there
// tidal-error-gcc: label .there. used but not defined
// tidal-error-edg: label .there. was referenced but not defined
void g() {
there:;
}
void f() {
  goto there;
}
