// tidal-rules: CMP-9
// tidal-expect: reject
// tidal-error-clang: comparison between pointer and integer
// tidal-error-gcc: comparison between pointer and integer
// tidal-error-edg: operand types are incompatible
bool f(int* p) {
  return p == 1;
}
