// tidal-rules: IF-6
// tidal-expect: reject
// tidal-error-clang: declaration in condition must have an initializer
// tidal-error-gcc: expected initializer before
// tidal-error-edg: condition declaration must include an initializer
void f() {
  if (int x) {}
}
