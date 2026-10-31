// tidal-rules: COMMA-6
// tidal-expect: reject
// tidal-error-clang: does not provide a subscript operator
// tidal-error-gcc: comma expression in array subscript
// tidal-error-edg: too many array bounds
int f(int* p, int i, int j) {
  return p[i, j];
}
