// tidal-rules: STO-5
// tidal-expect: reject
// tidal-error-clang: does not allow .register. storage class specifier
// tidal-error-gcc: does not allow .register. storage class specifier
// tidal-error-edg: register. storage class specifier is not allowed
void f() {
  register int x = 1;
  (void)x;
}
