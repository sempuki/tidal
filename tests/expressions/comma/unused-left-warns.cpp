// tidal-rules: COMMA-5
// tidal-expect: reject
// tidal-error-clang: left operand of comma operator has no effect
// tidal-error-gcc: left operand of comma operator has no effect
// tidal-flags: -Werror -Wunused-value
int f(int a, int b) {
  return (a + 1, b);
}
