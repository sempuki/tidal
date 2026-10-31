// tidal-rules: IF-10
// tidal-expect: reject
// tidal-error-clang: redefinition of .x
// tidal-error-gcc: redeclaration of .int x
// tidal-error-edg: declared in condition, may not be redeclared
void f() {
  if (int x = 1) {
    int x = 2;
  }
}
