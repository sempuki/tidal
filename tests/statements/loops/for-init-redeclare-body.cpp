// tidal-rules: LOOP-8
// tidal-expect: reject
// tidal-error-clang: redefinition of .i
// tidal-error-gcc: redeclaration of .int i
// tidal-error-edg: declared in for-loop initialization, may not be redeclared
void f() {
  for (int i = 0; i < 2; ++i) {
    int i = 5;
  }
}
