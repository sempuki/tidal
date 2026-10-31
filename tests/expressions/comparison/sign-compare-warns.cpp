// tidal-rules: CMP-4
// tidal-expect: reject
// tidal-error-clang: comparison of integers of different signs
// tidal-error-gcc: comparison of integer expressions of different signedness
// tidal-flags: -Werror -Wsign-compare
bool f(int a, unsigned b) {
  return a < b;
}
