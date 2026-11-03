// tidal-rules: LAM-23
// tidal-expect: reject
// tidal-error-clang: captured by copy in a non-mutable lambda
// tidal-error-gcc: assignment of read-only variable
// tidal-error-edg: must be a modifiable lvalue
void f() {
  auto g = [m = 1] { m = 2; };
}
