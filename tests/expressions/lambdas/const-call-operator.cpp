// tidal-rules: LAM-3
// tidal-expect: reject
// tidal-error-clang: captured by copy in a non-mutable lambda
// tidal-error-gcc: increment of read-only variable
// tidal-error-edg: must be a modifiable lvalue
void f() {
  int n = 0;
  auto g = [n] { ++n; };
}
