// tidal-rules: COMMA-9
// tidal-expect: reject
// tidal-error-clang: expected expression
// tidal-error-gcc: expected primary-expression before .\{
// tidal-error-edg: expected an expression
// tidal-note: The braced list is the left operand, in the condition of an if statement.
int pick() {
  if ({1}, true) return 1;
  return 0;
}
