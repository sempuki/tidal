// tidal-rules: FN-13
// tidal-expect: reject
// tidal-error-clang: redefinition of .x
// tidal-error-gcc: shadows a parameter
// tidal-error-edg: has already been declared in the current scope
void f(int x) {
  int x = 2;
}
