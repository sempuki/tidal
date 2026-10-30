// tidal-rules: COMMA-5
// tidal-expect: reject
// tidal-flags: -Werror -Wunused-value
int f(int a, int b) {
  return (a + 1, b);
}
