// tidal-rules: IF-6
// tidal-expect: reject
// tidal-error-clang: cannot be defined in a condition
// tidal-error-gcc: types may not be defined in conditions
// tidal-error-edg: type definition is not allowed
void f() {
  if (struct S { int v; explicit operator bool() const { return v; } } s{1}) {}
}
