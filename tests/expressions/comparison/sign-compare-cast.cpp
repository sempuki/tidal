// tidal-rules: CMP-4
// tidal-expect: accept
// tidal-flags: -Werror -Wsign-compare
bool f(int a, unsigned b) {
  return static_cast<unsigned>(a) < b;
}
