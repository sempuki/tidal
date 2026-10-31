// tidal-rules: IF-6
// tidal-expect: reject
// tidal-error-clang: array type is not allowed here
// tidal-error-gcc: condition declares an array
// tidal-error-edg: array type is not allowed here
void f() {
  if (int a[1] = {1}) {}
}
