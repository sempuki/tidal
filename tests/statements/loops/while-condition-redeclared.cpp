// tidal-rules: LOOP-4
// tidal-expect: reject
// tidal-error-clang: redefinition of .x
// tidal-error-gcc: redeclaration of .int x
// tidal-error-edg: declared in condition, may not be redeclared
// tidal-note: Gap review 2026-10-30.
void f() {
  while (int x = 0) {
    int x = 1;
  }
}
