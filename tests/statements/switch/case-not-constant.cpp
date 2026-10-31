// tidal-rules: SW-3
// tidal-expect: reject
// tidal-error-clang: case value is not a constant expression
// tidal-error-gcc: k. is not a constant expression
// tidal-error-edg: expression must have a constant value
void f(int v, int k) {
  switch (v) { case k: break; }
}
