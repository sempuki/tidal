// tidal-rules: CMP-9
// tidal-expect: reject
// tidal-error-clang: comparison of distinct pointer types
// tidal-error-gcc: comparison between distinct pointer types
// tidal-error-edg: operand types are incompatible
bool f(int* p, double* q) {
  return p == q;
}
