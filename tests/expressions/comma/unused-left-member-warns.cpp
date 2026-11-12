// tidal-rules: COMMA-5 UNEV-3
// tidal-expect: reject
// tidal-error-clang: left operand of comma operator has no effect
// tidal-error-gcc: left operand of comma operator has no effect
// tidal-flags: -Werror -Wunused-value
// tidal-note: X26-3. The left operand reads a member that is not volatile, which has no effect.
struct Point { int x, y; };
int second(Point p) {
  return (p.x, p.y);
}
