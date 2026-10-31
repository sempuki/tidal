// tidal-rules: CV-4
// tidal-expect: reject
// tidal-error-clang: with const-qualified type
// tidal-error-gcc: increment of read-only variable
// tidal-error-edg: must be a modifiable lvalue
void f() {
  const int x = 1;
  ++x;
}
