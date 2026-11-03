// tidal-rules: RFOR-6
// tidal-expect: reject
// tidal-error-clang: redefinition of .v
// tidal-error-gcc: redeclaration of .double v
// tidal-error-edg: declared as iterator of range-based .for. statement, may not be redeclared
// tidal-note: The loop variable is a reference, and the body declares a double with the same name.
void f() {
  int a[2] = {1, 2};
  for (auto& v : a) {
    double v = 3;
  }
}
