// tidal-rules: IF-10
// tidal-expect: reject
// tidal-error-clang: redefinition of .x
// tidal-error-gcc: redeclaration of .int x
// tidal-error-edg: x. has already been declared in the current scope
void f() {
  if (int x = 1; int x = 2) {}
}
