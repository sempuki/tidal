// tidal-rules: STO-7
// tidal-expect: reject
// tidal-error-clang: identifier with linkage cannot have an initializer
// tidal-error-gcc: has both .extern. and initializer
// tidal-error-edg: initializer is not allowed on a local declaration of an extern variable
void f() {
  extern int x = 1;
}
