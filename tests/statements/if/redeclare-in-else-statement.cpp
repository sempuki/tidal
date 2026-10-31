// tidal-rules: IF-10
// tidal-expect: reject
// tidal-error-clang: redefinition of .x
// tidal-error-gcc: redeclaration of .int x
// tidal-error-edg: declared in condition, may not be redeclared
void f() {
  if (auto x = 1) {} else int x = 2;
}
