// tidal-rules: CMP-4
// tidal-expect: reject
// tidal-flags: -Werror -Wsign-compare
bool f(int a, unsigned b) {
  return a < b;
}
