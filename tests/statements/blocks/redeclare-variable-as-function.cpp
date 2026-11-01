// tidal-rules: STMT-8
// tidal-expect: reject
// tidal-error-clang: redefinition of .value. as different kind of symbol
// tidal-error-gcc: redeclared as different kind of entity
// tidal-error-edg: value. has already been declared in the current scope
void f() {
  int value = 1;
  void value();
}
