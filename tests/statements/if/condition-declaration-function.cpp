// tidal-rules: IF-6
// tidal-expect: reject
// tidal-error-clang: function type is not allowed here
// tidal-error-gcc: condition declares a function
// tidal-error-edg: condition declaration must declare a variable
// tidal-note: Gap review 2026-10-30.
void f() {
  if (int g() = 0) {}
}
