// tidal-rules: COMMA-5
// tidal-expect: accept
// tidal-flags: -Werror -Wunused-value
int f(int a, int b) {
  return ((void)(a + 1), b);
}
