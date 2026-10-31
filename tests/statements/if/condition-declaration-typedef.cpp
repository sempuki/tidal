// tidal-rules: IF-6
// tidal-expect: reject
// tidal-error-clang: does not allow storage class
// tidal-error-gcc: typedef. invalid in condition
// tidal-error-edg: condition declaration must declare a variable
// tidal-note: Gap review 2026-10-30.
void f() {
  if (typedef int T = 1) {}
}
