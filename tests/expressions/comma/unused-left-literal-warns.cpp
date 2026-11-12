// tidal-rules: COMMA-5 UNEV-3
// tidal-expect: reject
// tidal-error-clang: left operand of comma operator has no effect
// tidal-error-gcc: left operand of comma operator has no effect
// tidal-flags: -Werror -Wunused-value
// tidal-note: X26-3. The left operand is a literal, which has no effect.
int pick(int b) {
  return (7, b);
}
