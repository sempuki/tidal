// tidal-rules: STMT-8
// tidal-expect: reject
// tidal-error-clang: redefinition of .x. with a different type
// tidal-error-gcc: conflicting declaration .double x
// tidal-error-edg: x. has already been declared in the current scope
void f() {
  int x = 1;
  double x = 2.0;
}
