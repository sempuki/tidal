// tidal-rules: DCL-19
// tidal-expect: reject
// tidal-error-clang: variable length arrays
// tidal-error-gcc: forbids variable length array
// tidal-error-edg: must have a constant value
void f(int n) {
  int a[n];
  (void)a;
}
