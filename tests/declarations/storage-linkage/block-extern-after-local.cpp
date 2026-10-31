// tidal-rules: STO-7
// tidal-expect: reject
// tidal-error-clang: extern declaration of .x. follows non-extern declaration
// tidal-error-gcc: conflicts with a previous declaration
// tidal-error-edg: has already been declared in the current scope
void f() {
  int x = 0;
  extern int x;
}
