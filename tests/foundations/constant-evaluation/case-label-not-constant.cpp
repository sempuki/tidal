// tidal-rules: CE-11
// tidal-expect: reject
// tidal-error-clang: case value is not a constant expression
// tidal-error-gcc: is not a constant expression
// tidal-error-edg: expression must have a constant value
int pick(int v, int key) {
  switch (v) {
  case key:
    return 1;
  }
  return 0;
}
