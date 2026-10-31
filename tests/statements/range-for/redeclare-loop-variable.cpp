// tidal-rules: RFOR-6
// tidal-expect: reject
// tidal-error-clang: redefinition of .v
// tidal-error-gcc: redeclaration of .int v
// tidal-error-edg: declared as iterator of range-based .for. statement, may not be redeclared
// tidal-note: Gap review 2026-10-30.
void f() {
  int a[2] = {1, 2};
  for (int v : a) {
    int v = 3;
  }
}
